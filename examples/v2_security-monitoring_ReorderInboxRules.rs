// Reorder the list of inbox rules in the pipeline returns "The list of inbox
// rules" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::InboxRulesType;
use datadog_api_client::datadogV2::model::ReorderInboxRulesParameters;
use datadog_api_client::datadogV2::model::ReorderInboxRulesParametersData;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ReorderInboxRulesParameters::new().data(vec![ReorderInboxRulesParametersData::new(
        Uuid::parse_str("123e4567-e89b-12d3-a456-426655440000").expect("invalid UUID"),
        InboxRulesType::INBOX_RULES,
    )]);
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.reorder_inbox_rules(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
