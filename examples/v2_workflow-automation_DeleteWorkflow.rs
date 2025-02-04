// Delete an existing Workflow returns "Successfully deleted a workflow." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "workflow" in the system
    let workflow_data_id = std::env::var("WORKFLOW_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.delete_workflow(workflow_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
