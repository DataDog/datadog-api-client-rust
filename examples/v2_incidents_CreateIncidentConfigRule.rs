// Create incident rule returns "Created" response
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
    configuration.set_unstable_operation_enabled("v2.CreateIncidentConfigRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_config_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
