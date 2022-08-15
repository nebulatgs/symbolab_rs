use anyhow::Context;
use axum::{routing::post, Extension, Json, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, oneshot, RwLock};
use tower_http::trace::TraceLayer;
use tracing::{error, info, instrument, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
use error::*;

mod symbolab;
use symbolab::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "symbolab_rs=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = Client::new();
    let (tx, mut rx) = mpsc::channel(20);
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            let res = token_factory(&mut rx).await;
            match res {
                Err(e) => {
                    error!("{:#}", e);
                }
                Ok(_) => {
                    unreachable!("token_factory should never return Ok");
                }
            }
            error!("factory died! (reboot count: {count})");
            count += 1;
        }
    });

    let app = Router::new()
        .route("/", post(handler))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(State {
            client,
            token_channel: tx,
            response_cache: Arc::new(RwLock::new(HashMap::new())),
        }));

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT").unwrap_or("8080".to_owned()).parse()?,
    ));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[instrument(skip_all)]
async fn token_factory(rx: &mut mpsc::Receiver<oneshot::Sender<String>>) -> anyhow::Result<()> {
    const TOKEN_CAPACITY: usize = 10;
    let queue_len = Arc::new(AtomicUsize::new(0));

    let client = Client::new();
    let (tx_token, mut rx_token) = mpsc::channel(TOKEN_CAPACITY);
    let (tx_internal, mut rx_internal) = mpsc::channel(TOKEN_CAPACITY);

    {
        let queue_len = queue_len.clone();
        tokio::spawn(async move {
            while let Some(()) = rx_internal.recv().await {
                queue_len.fetch_add(1, Ordering::Relaxed);
                tx_token.send(get_token(&client).await?).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    info!("starting");
    for _ in 0..TOKEN_CAPACITY {
        tx_internal.send(()).await?;
    }
    info!("queued {TOKEN_CAPACITY} tokens");

    while let Some(channel) = rx.recv().await {
        queue_len.fetch_sub(1, Ordering::Relaxed);
        if queue_len.load(Ordering::Relaxed) == 0 {
            warn!("ran out of tokens!");
        }
        if let Some(token) = rx_token.recv().await {
            match channel.send(token) {
                Ok(_) => {}
                Err(e) => {
                    error!("failed to send token: `{}`", e);
                }
            }
            tx_internal.send(()).await?;
        } else {
            return Err(anyhow::anyhow!("all channels closed"));
        }
    }

    Err(anyhow::anyhow!("all channels closed"))
}

#[derive(Debug, Clone)]
struct State {
    client: Client,
    token_channel: mpsc::Sender<oneshot::Sender<String>>,
    response_cache: Arc<RwLock<HashMap<Payload, Data>>>,
}

async fn get_token(client: &Client) -> anyhow::Result<String> {
    let res = client
        .get("https://www.symbolab.com/solver/step-by-step/")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36")
        .send()
        .await?;

    let set_cookie = res
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|h| h.to_str())
        .collect::<core::result::Result<Vec<_>, _>>()?;

    let token = set_cookie
        .iter()
        .find_map(|s| {
            s.split("; ")
                .find_map(|pair| {
                    pair.split(", ")
                        .map(|p2| p2.split('='))
                        .map(|mut arr| (arr.next(), arr))
                        .find(|(first, _)| *first == Some("sy2.pub.token"))
                })?
                .1
                .next()
        })
        .context("No token")?;

    Ok(token.to_owned())
}

async fn get_symbolab(
    state: &State,
    token: &str,
    payload: &Payload,
) -> anyhow::Result<SymbolabResponse> {
    let res = state.client
    .get("https://www.symbolab.com/pub_api/steps")
    .query(payload)
    .query(&[
        ("subscribed", "false"), 
        ("language", "en"), 
        ("plotRequest", "PlotOptional"), 
        ("page", "step-by-step")
    ])
    .bearer_auth(token)
    .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36")
    // .header("sec-ch-ua", r#"" Not A;Brand";v="99", "Chromium";v="90", "Google Chrome";v="90""#)
    // .header("referer", "https://www.symbolab.com/solver/step-by-step/x")
    // .header("cache-control", "no-cache")
    // .header("pragma", "no-cache")
    // .header("sec-ch-ua-mobile", "?0")
    // .header("sec-fetch-dest", "empty")
    // .header("sec-fetch-mode", "cors")
    // .header("sec-fetch-site", "same-origin")
    .header("x-requested-with", "XMLHttpRequest")
    .send()
    .await?;
    let symbolab: SymbolabResponse = res.json().await?;
    Ok(symbolab)
}

async fn get_cached_token(state: &State) -> anyhow::Result<String> {
    let (tx, rx) = oneshot::channel();
    state.token_channel.send(tx).await?;
    Ok(rx.await?)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
struct Payload {
    query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    symbolab: SymbolabResponse,
    cached: bool,
}

async fn handler(
    Json(payload): Json<Payload>,
    Extension(state): Extension<State>,
) -> Result<Json<Data>> {
    {
        let reader = state.response_cache.read().await;
        if let Some(data) = reader.get(&payload) {
            return Ok(Json(data.clone()));
        }
    }

    let token = get_cached_token(&state).await?;
    let symbolab = get_symbolab(&state, &token, &payload).await?;
    let data = Data {
        symbolab,
        cached: false,
    };

    {
        let mut data = data.clone();
        data.cached = true;
        tokio::spawn(async move {
            let mut writer = state.response_cache.write().await;
            writer.insert(payload, data);
        });
    }

    Ok(Json(data))
}
