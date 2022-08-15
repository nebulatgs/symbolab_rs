use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolabResponse {
    pub dym: Option<Dym>,
    #[serde(rename = "dymAlternatives")]
    pub dym_alternatives: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "relatedQueries")]
    pub related_queries: Option<Vec<RelatedQuery>>,
    #[serde(rename = "relatedProblems")]
    pub related_problems: Option<Vec<String>>,
    #[serde(rename = "standardQuery")]
    pub standard_query: Option<String>,
    #[serde(rename = "stepLang")]
    pub step_lang: Option<String>,
    #[serde(rename = "isFromCache")]
    pub is_from_cache: Option<bool>,
    #[serde(rename = "isInNotebook")]
    pub is_in_notebook: Option<bool>,
    #[serde(rename = "showVerify")]
    pub show_verify: Option<bool>,
    #[serde(rename = "showViewLarger")]
    pub show_view_larger: Option<bool>,
    #[serde(rename = "canonicalNotebookQuery")]
    pub canonical_notebook_query: Option<String>,
    pub subject: Option<String>,
    pub topic: Option<String>,
    #[serde(rename = "subTopic")]
    pub sub_topic: Option<String>,
    pub solutions: Option<Vec<SolutionElement>>,
    #[serde(rename = "plotInfo")]
    pub plot_info: Option<PlotInfo>,
    #[serde(rename = "solutionLevel")]
    pub solution_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dym {
    #[serde(rename = "inputEquation")]
    pub input_equation: Option<String>,
    #[serde(rename = "originalEquation")]
    pub original_equation: Option<String>,
    #[serde(rename = "originalText")]
    pub original_text: Option<String>,
    #[serde(rename = "outEquation")]
    pub out_equation: Option<String>,
    #[serde(rename = "outText")]
    pub out_text: Option<String>,
    #[serde(rename = "dymEquation")]
    pub dym_equation: Option<String>,
    #[serde(rename = "dymText")]
    pub dym_text: Option<String>,
    #[serde(rename = "isTemplate")]
    pub is_template: Option<bool>,
    #[serde(rename = "showDidYouMean")]
    pub show_did_you_mean: Option<bool>,
    #[serde(rename = "showInstead")]
    pub show_instead: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotInfo {
    pub variable: Option<String>,
    #[serde(rename = "linesToDraw")]
    pub lines_to_draw: Option<Vec<Option<serde_json::Value>>>,
    pub fills: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "functionChanges")]
    pub function_changes: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "graphCalcInputErrors")]
    pub graph_calc_input_errors: Option<Vec<Option<serde_json::Value>>>,
    #[serde(rename = "plotRequest")]
    pub plot_request: Option<String>,
    #[serde(rename = "isInCache")]
    pub is_in_cache: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedQuery {
    pub command: Option<String>,
    pub equation: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionElement {
    pub solution: Option<SolutionSolution>,
    pub step_input: Option<String>,
    pub entire_result: Option<String>,
    #[serde(rename = "solvingClass")]
    pub solving_class: Option<String>,
    #[serde(rename = "isInterimStep")]
    pub is_interim_step: Option<bool>,
    #[serde(rename = "isOpen")]
    pub is_open: Option<bool>,
    #[serde(rename = "isShowSolutionAfterStep")]
    pub is_show_solution_after_step: Option<bool>,
    pub title: Option<Title>,
    pub steps: Option<Vec<Step>>,
    #[serde(rename = "practiceLink")]
    pub practice_link: Option<String>,
    #[serde(rename = "practiceTopic")]
    pub practice_topic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionSolution {
    #[serde(rename = "apiTitle")]
    pub api_title: Option<Title>,
    #[serde(rename = "default")]
    pub solution_default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub text: Option<Text>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "createdText")]
    pub created_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub step_input: Option<String>,
    pub entire_result: Option<String>,
    #[serde(rename = "isInterimStep")]
    pub is_interim_step: Option<bool>,
    #[serde(rename = "isOpen")]
    pub is_open: Option<bool>,
    #[serde(rename = "isShowSolutionAfterStep")]
    pub is_show_solution_after_step: Option<bool>,
    pub title: Option<Title>,
    pub general_rule: Option<Title>,
}
