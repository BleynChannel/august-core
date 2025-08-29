use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

//? Maybe replace with petgraph?
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGraph {
    // Simplified representation of a task graph
    pub nodes: Vec<String>,
    pub edges: Vec<(usize, usize)>,
}

#[derive(Error, Debug)]
pub enum PlannerError {
    #[error("Planning failed: {0}")]
    PlanningFailed(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn plan(&self, nlp_response: super::nlp::NlpResponse) -> Result<TaskGraph, PlannerError> {
        // Simulate planning logic
        info!("Planning tasks based on NLP response: {:?}", nlp_response);
        Ok(TaskGraph {
            nodes: vec![],
            edges: vec![], // For simplicity, no dependencies in mock
        })
    }
}
