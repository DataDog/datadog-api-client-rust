// Get incident rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentConfigRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .get_incident_config_rule("612e0c88-9137-4bd2-8de4-9356867d4c6a".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
