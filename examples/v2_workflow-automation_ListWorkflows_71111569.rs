// List workflows returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::ListWorkflowsOptionalParams;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = WorkflowAutomationAPI::with_config(configuration);
    let response = api.list_workflows_with_pagination(
        ListWorkflowsOptionalParams::default()
            .filter_query("Example-Workflow-Automation".to_string())
            .limit(2),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
