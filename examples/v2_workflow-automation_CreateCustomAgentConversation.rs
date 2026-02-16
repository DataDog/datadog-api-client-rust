// Create a custom agent conversation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::CustomAgentConversationRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CustomAgentConversationRequest::new("What is the weather like today?".to_string())
        .conversation_id("550e8400-e29b-41d4-a716-446655440000".to_string());
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateCustomAgentConversation", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api
        .create_custom_agent_conversation(
            Uuid::parse_str("3b796bda-b79b-477e-ae29-958473a683db").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
