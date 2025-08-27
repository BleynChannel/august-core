use plux::{Loader, StdInfo, function::FunctionOutput};

use crate::{
    context::ContextManager, executor::Executor, nlp::{MockNlpProcessor, NlpProcessor, NlpRequest}, planner::Planner, storage::Storage, Config, CoreError
};

pub struct Core {
    config: Config,
    storage: Storage,
    plux: Loader<'static, FunctionOutput, StdInfo>,
    nlp_processor: Box<dyn NlpProcessor>,
    planner: Planner,
    executor: Executor,
    context_manager: ContextManager,
}

impl Core {
    pub fn new(config: Config) -> Result<Self, CoreError> {
        Ok(Self {
            config,
            plux: Loader::new(),
            nlp_processor: Box::new(MockNlpProcessor),
            planner: Planner::new(),
            executor: Executor::new(),
            context_manager: ContextManager::new(100), // TODO: make it configurable
            storage: Storage::new("".into())?,
        })
    }

    pub async fn process(&mut self, input: &str) -> Result<serde_json::Value, CoreError> {
        // 1. NLP Processing
        let nlp_request = NlpRequest { text: input.to_string() };
        let nlp_response = self.nlp_processor.process(nlp_request).await?;

        // 2. Task Planning
        let task_graph = self.planner.plan(nlp_response)?;

        // 3. Task Execution
        let execution_results = self.executor.execute(task_graph).await?;

        // 4. Update Context and Storage (simplified)
        let user_id = "default_user"; // For demonstration
        self.context_manager.add_to_history(user_id, input.to_string());
        self.storage.insert("last_command_result", serde_json::json!(execution_results).to_string()).await?;

        Ok(serde_json::json!({ "status": "success", "input": input, "results": execution_results }))
    }
}
