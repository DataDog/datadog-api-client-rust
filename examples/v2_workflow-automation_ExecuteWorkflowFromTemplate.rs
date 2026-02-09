// Execute a workflow from a template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::WorkflowHeadlessExecutionConfig;
use datadog_api_client::datadogV2::model::WorkflowHeadlessExecutionConnection;
use datadog_api_client::datadogV2::model::WorkflowHeadlessExecutionRequest;
use datadog_api_client::datadogV2::model::WorkflowHeadlessExecutionRequestAttributes;
use datadog_api_client::datadogV2::model::WorkflowHeadlessExecutionRequestData;
use datadog_api_client::datadogV2::model::WorkflowHeadlessExecutionRequestType;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = WorkflowHeadlessExecutionRequest::new(WorkflowHeadlessExecutionRequestData::new(
        WorkflowHeadlessExecutionRequestAttributes::new(
            WorkflowHeadlessExecutionConfig::new(
                vec![WorkflowHeadlessExecutionConnection::new(
                    Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("invalid UUID"),
                    "INTEGRATION_DATADOG".to_string(),
                )],
                BTreeMap::from([]),
            ),
            "template-789".to_string(),
        ),
        "1234".to_string(),
        WorkflowHeadlessExecutionRequestType::WORKFLOW_HEADLESS_EXECUTION_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ExecuteWorkflowFromTemplate", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api
        .execute_workflow_from_template("parent_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
