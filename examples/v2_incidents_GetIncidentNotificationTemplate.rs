// Get incident notification template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::GetIncidentNotificationTemplateOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "notification_template" in the system
    let notification_template_data_id =
        uuid::Uuid::parse_str(&std::env::var("NOTIFICATION_TEMPLATE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentNotificationTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .get_incident_notification_template(
            notification_template_data_id.clone(),
            GetIncidentNotificationTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
