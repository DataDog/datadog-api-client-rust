// Delete incident notification rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::DeleteIncidentNotificationRuleOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "notification_rule" in the system
    let notification_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("NOTIFICATION_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteIncidentNotificationRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .delete_incident_notification_rule(
            notification_rule_data_id.clone(),
            DeleteIncidentNotificationRuleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
