// Reorder the list of mute rules in the pipeline returns "The list of mute rules"
// response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::MuteRulesType;
use datadog_api_client::datadogV2::model::ReorderMuteRulesParameters;
use datadog_api_client::datadogV2::model::ReorderMuteRulesParametersData;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ReorderMuteRulesParameters::new().data(vec![ReorderMuteRulesParametersData::new(
        Uuid::parse_str("123e4567-e89b-12d3-a456-426655440000").expect("invalid UUID"),
        MuteRulesType::MUTE_RULES,
    )]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.reorder_mute_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
