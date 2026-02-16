// Generate workflow description returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::WorkflowDescriptionRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        WorkflowDescriptionRequest::new("Alert Response Workflow".to_string(), BTreeMap::new());
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateWorkflowDescription", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.create_workflow_description(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
