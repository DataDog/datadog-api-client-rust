// Delete a mute rule returns "Rule successfully deleted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "valid_mute_rule" in the system
    let valid_mute_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("VALID_MUTE_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.delete_mute_rule(valid_mute_rule_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
