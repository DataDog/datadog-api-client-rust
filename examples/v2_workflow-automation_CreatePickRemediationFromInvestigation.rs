// Pick remediation actions from investigation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_workflow_automation::WorkflowAutomationAPI;
use datadog_api_client::datadogV2::model::ClientType;
use datadog_api_client::datadogV2::model::PickRemediationFromInvestigationRequest;
use datadog_api_client::datadogV2::model::StabilityLevel;

#[tokio::main]
async fn main() {
    let body = PickRemediationFromInvestigationRequest::new(
        "High CPU usage detected on prod-server-01".to_string(),
        5,
    )
    .client(ClientType::WORKFLOWS)
    .integrations(vec!["aws".to_string(), "datadog".to_string()])
    .number_of_keyword_variants(2)
    .stability(StabilityLevel::STABLE);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreatePickRemediationFromInvestigation", true);
    let api = WorkflowAutomationAPI::with_config(configuration);
    let resp = api.create_pick_remediation_from_investigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
