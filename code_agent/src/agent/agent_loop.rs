use super::llm_client::{LlmClient, Message, ToolCall};
use super::tool_registry::ToolRegistry;
use crate::tools::ToolParams;
use anyhow::{Context, Result};
use colored::*;
use serde_json;

pub struct AgentLoop {
    client: LlmClient,
    registry: ToolRegistry,
    messages: Vec<Message>,
    max_iterations: usize,
    verbose: bool,
}

impl AgentLoop {
    pub fn new(verbose: bool) -> Result<Self> {
        Ok(Self {
            client: LlmClient::new()?,
            registry: ToolRegistry::new(),
            messages: Vec::new(),
            max_iterations: 25,
            verbose,
        })
    }

    pub fn set_system_prompt(&mut self, system_prompt: String) {
        self.messages.push(Message {
            role: "system".to_string(),
            content: system_prompt,
            tool_calls: None,
            tool_call_id: None,
        });
    }

    pub async fn run(&mut self, user_prompt: String) -> Result<String> {
        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_prompt.clone(),
            tool_calls: None,
            tool_call_id: None,
        });

        if self.verbose {
            println!("{} {}", "User:".bright_cyan().bold(), user_prompt);
            println!();
        }

        let mut iteration = 0;
        let tool_definitions = self.registry.get_tool_definitions();

        loop {
            iteration += 1;
            if iteration > self.max_iterations {
                anyhow::bail!("Maximum iterations ({}) exceeded", self.max_iterations);
            }

            if self.verbose {
                println!(
                    "{} {}",
                    "Iteration:".dimmed(),
                    format!("{}/{}", iteration, self.max_iterations).dimmed()
                );
            }

            // Call LLM
            let response = self
                .client
                .chat(self.messages.clone(), Some(tool_definitions.clone()), Some(4096))
                .await
                .context("Failed to get LLM response")?;

            // Check if LLM wants to use tools
            if let Some(tool_calls) = &response.tool_calls {
                if self.verbose {
                    println!(
                        "{} {} tool call(s)",
                        "Assistant:".bright_green().bold(),
                        tool_calls.len()
                    );
                }

                // Add assistant's message with tool calls to history
                self.messages.push(response.clone());

                // Execute each tool call
                for tool_call in tool_calls {
                    self.execute_tool_call(tool_call).await?;
                }
            } else {
                // No more tool calls, return final response
                if self.verbose {
                    println!("{}", "Assistant:".bright_green().bold());
                    println!("{}", response.content);
                    println!();
                }

                // Add final response to history
                self.messages.push(response.clone());

                return Ok(response.content);
            }
        }
    }

    async fn execute_tool_call(&mut self, tool_call: &ToolCall) -> Result<()> {
        let tool_name = &tool_call.function.name;
        let arguments = &tool_call.function.arguments;

        if self.verbose {
            println!(
                "  {} {}",
                "→".bright_yellow(),
                format!("{}({})", tool_name, arguments).dimmed()
            );
        }

        // Parse arguments as JSON
        let args: serde_json::Value = serde_json::from_str(arguments)
            .context(format!("Failed to parse tool arguments: {}", arguments))?;

        let params = ToolParams { data: args };

        // Execute the tool
        let result = self.registry.execute_tool(tool_name, params)?;

        // Format result message
        let result_content = if result.success {
            if self.verbose {
                println!(
                    "  {} {}",
                    "✓".bright_green(),
                    result.output.lines().next().unwrap_or("Success")
                );
            }
            result.output
        } else {
            if self.verbose {
                println!(
                    "  {} {}",
                    "✗".bright_red(),
                    result.error.as_ref().unwrap_or(&"Unknown error".to_string())
                );
            }
            format!("Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()))
        };

        // Add tool result to messages
        self.messages.push(Message {
            role: "tool".to_string(),
            content: result_content,
            tool_calls: None,
            tool_call_id: Some(tool_call.id.clone()),
        });

        Ok(())
    }

    pub fn get_conversation_history(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear_history(&mut self) {
        self.messages.clear();
    }
}
