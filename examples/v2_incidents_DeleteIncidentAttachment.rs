// Delete incident attachment returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentAttachment", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_attachment(
            "incident_id".to_string(),
            Value::from("00000000-0000-0000-0000-000000000002"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
