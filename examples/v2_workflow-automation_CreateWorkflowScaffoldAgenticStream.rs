// Generate workflow scaffold with agentic stream returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::ChatMessage;
use datadog_api_client::datadogV2::model::ChatMessageRole;
use datadog_api_client::datadogV2::model::UserContext;
use datadog_api_client::datadogV2::model::UserInfo;
use datadog_api_client::datadogV2::model::WorkflowScaffoldAgenticStreamRequest;

#[tokio::main]
async fn main() {
    let body = WorkflowScaffoldAgenticStreamRequest::new(
        "Create a workflow to restart a service when CPU is high".to_string(),
    )
    .chat_history(vec![ChatMessage::new(
        "chat-456".to_string(),
        "Add error handling to the workflow".to_string(),
        "msg-123".to_string(),
        ChatMessageRole::USER,
    )
    .user_uuid("550e8400-e29b-41d4-a716-446655440000".to_string())])
    .previous_action("created_initial_scaffold".to_string())
    .user_context(UserContext::new(
        UserInfo::new(
            "Acme Corp".to_string(),
            "john.doe@example.com".to_string(),
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
        )
        .user_name("John Doe".to_string()),
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateWorkflowScaffoldAgenticStream", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.create_workflow_scaffold_agentic_stream(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
