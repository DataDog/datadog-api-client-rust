// Update incident rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentRuleAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentRuleDataRequest;
use datadog_api_client::datadogV2::model::IncidentRuleRequest;
use datadog_api_client::datadogV2::model::IncidentRuleType;

#[tokio::main]
async fn main() {
    let body = IncidentRuleRequest::new(IncidentRuleDataRequest::new(
        IncidentRuleAttributesRequest::new("High Severity Rule".to_string()),
        IncidentRuleType::INCIDENT_RULE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentConfigRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_config_rule("612e0c88-9137-4bd2-8de4-9356867d4c6a".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
