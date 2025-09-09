// Delete a notification template returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::DeleteIncidentNotificationTemplateOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentNotificationTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_notification_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("invalid UUID"),
            DeleteIncidentNotificationTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
