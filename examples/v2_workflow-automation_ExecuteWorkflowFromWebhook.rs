// Execute a workflow from a webhook returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = BTreeMap::from([("foo".to_string(), Value::from("bar"))]);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ExecuteWorkflowFromWebhook", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api
        .execute_workflow_from_webhook(
            "workflow_id".to_string(),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            "sha256=abcdef123456...".to_string(),
            "GitHub-Hookshot/abc123".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
