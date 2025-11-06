use super::{Tool, ToolOutput};
use crate::llm::{LLMProvider, LLMRequest};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;

pub struct AgentExecutor {
    llm: Arc<dyn LLMProvider>,
    tools: HashMap<String, Arc<dyn Tool>>,
    max_iterations: usize,
}

impl AgentExecutor {
    pub fn new(llm: Arc<dyn LLMProvider>, max_iterations: usize) -> Self {
        Self {
            llm,
            tools: HashMap::new(),
            max_iterations,
        }
    }
    
    pub fn add_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }
    
    pub async fn execute(&self, task: &str) -> Result<AgentResult> {
        let mut steps = Vec::new();
        let mut current_input = task.to_string();
        
        for iteration in 0..self.max_iterations {
            let step_start = std::time::Instant::now();
            
            // Ask LLM to decide what to do
            let reasoning_prompt = self.build_reasoning_prompt(&current_input, iteration);
            let request = LLMRequest::new(reasoning_prompt);
            let response = self.llm.generate(&request).await?;
            
            // Parse LLM response to extract action
            let action = self.parse_action(&response.text)?;
            
            let step_result = if action.action_type == "final_answer" {
                // Agent has finished
                steps.push(AgentStep {
                    iteration,
                    thought: action.thought.clone(),
                    action: action.action_type.clone(),
                    action_input: action.action_input.clone(),
                    observation: action.action_input.clone(),
                    duration_ms: step_start.elapsed().as_millis() as u64,
                });
                
                return Ok(AgentResult {
                    final_answer: action.action_input,
                    steps,
                    total_iterations: iteration + 1,
                });
            } else if let Some(tool) = self.tools.get(&action.action_type) {
                // Execute tool
                let tool_result = tool.execute(&action.action_input).await?;
                
                steps.push(AgentStep {
                    iteration,
                    thought: action.thought.clone(),
                    action: action.action_type.clone(),
                    action_input: action.action_input.clone(),
                    observation: tool_result.result.clone(),
                    duration_ms: step_start.elapsed().as_millis() as u64,
                });
                
                current_input = tool_result.result;
                current_input
            } else {
                return Err(anyhow::anyhow!("Unknown tool: {}", action.action_type));
            };
        }
        
        Err(anyhow::anyhow!("Agent exceeded maximum iterations"))
    }
    
    fn build_reasoning_prompt(&self, input: &str, iteration: usize) -> String {
        let tools_desc = self.tools
            .values()
            .map(|t| format!("- {}: {}", t.name(), t.description()))
            .collect::<Vec<_>>()
            .join("\n");
        
        format!(
            r#"You are an AI agent that can use tools to accomplish tasks.

Available tools:
{}

Task: {}

Current iteration: {}

Respond in this format:
Thought: [your reasoning about what to do next]
Action: [tool name or "final_answer"]
Action Input: [input for the tool or your final answer]

Begin!"#,
            tools_desc, input, iteration
        )
    }
    
    fn parse_action(&self, response: &str) -> Result<AgentAction> {
        // Simple parsing (in production, use proper parsing)
        let mut thought = String::new();
        let mut action_type = String::new();
        let mut action_input = String::new();
        
        for line in response.lines() {
            if line.starts_with("Thought:") {
                thought = line.trim_start_matches("Thought:").trim().to_string();
            } else if line.starts_with("Action:") {
                action_type = line.trim_start_matches("Action:").trim().to_string();
            } else if line.starts_with("Action Input:") {
                action_input = line.trim_start_matches("Action Input:").trim().to_string();
            }
        }
        
        if action_type.is_empty() {
            action_type = "final_answer".to_string();
            action_input = response.to_string();
        }
        
        Ok(AgentAction {
            thought,
            action_type,
            action_input,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub thought: String,
    pub action_type: String,
    pub action_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStep {
    pub iteration: usize,
    pub thought: String,
    pub action: String,
    pub action_input: String,
    pub observation: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub final_answer: String,
    pub steps: Vec<AgentStep>,
    pub total_iterations: usize,
}
