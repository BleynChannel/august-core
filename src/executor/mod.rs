use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: i64,
    pub status: String, // TODO: Enum
    pub output: Option<serde_json::Value>,
}

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(
        &self,
        task_graph: super::planner::TaskGraph,
    ) -> Result<HashMap<i64, TaskResult>, ExecutorError> {
        info!("Executing task graph: {:?}", task_graph);
        let mut results = HashMap::new();
        for (i, node) in task_graph.nodes.iter().enumerate() {
            // Simulate task execution
            let task_id = i as i64;
            info!("Executing task: {}", node);
            results.insert(
                task_id.clone(),
                TaskResult {
                    task_id: task_id,
                    status: "completed".to_string(),
                    output: Some(
                        serde_json::json!({ "message": format!("Task '{}' completed", node) }),
                    ),
                },
            );
        }
        Ok(results)
    }
}
