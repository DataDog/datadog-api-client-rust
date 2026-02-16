// Pick relevant actions returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::ClientType;
use datadog_api_client::datadogV2::model::PickActionRequest;
use datadog_api_client::datadogV2::model::StabilityLevel;

#[tokio::main]
async fn main() {
    let body = PickActionRequest::new(5, "Send a Slack message".to_string())
        .client(ClientType::WORKFLOWS)
        .stability(StabilityLevel::STABLE);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreatePickAction", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.create_pick_action(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
