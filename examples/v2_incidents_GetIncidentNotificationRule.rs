// Get an incident notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::GetIncidentNotificationRuleOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIncidentNotificationRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .get_incident_notification_rule(
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("invalid UUID"),
            GetIncidentNotificationRuleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
