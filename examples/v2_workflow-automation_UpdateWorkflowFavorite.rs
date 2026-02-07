// Update workflow favorite status returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::WorkflowFavoriteRequest;
use datadog_api_client::datadogV2::model::WorkflowFavoriteRequestAttributes;
use datadog_api_client::datadogV2::model::WorkflowFavoriteRequestData;
use datadog_api_client::datadogV2::model::WorkflowFavoriteRequestType;

#[tokio::main]
async fn main() {
    let body = WorkflowFavoriteRequest::new(WorkflowFavoriteRequestData::new(
        WorkflowFavoriteRequestAttributes::new(true),
        WorkflowFavoriteRequestType::WORKFLOW_FAVORITE_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateWorkflowFavorite", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api
        .update_workflow_favorite("workflow_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
