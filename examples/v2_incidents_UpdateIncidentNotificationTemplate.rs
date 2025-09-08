// Update incident notification template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentNotificationTemplateOptionalParams;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateType;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateUpdateAttributes;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateUpdateData;
use datadog_api_client::datadogV2::model::PatchIncidentNotificationTemplateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "notification_template" in the system
    let notification_template_data_id =
        uuid::Uuid::parse_str(&std::env::var("NOTIFICATION_TEMPLATE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = PatchIncidentNotificationTemplateRequest::new(
        IncidentNotificationTemplateUpdateData::new(
            notification_template_data_id.clone(),
            IncidentNotificationTemplateType::NOTIFICATION_TEMPLATES,
        )
        .attributes(
            IncidentNotificationTemplateUpdateAttributes::new()
                .category("update".to_string())
                .content(
                    r#"Incident Status Update:

Title: Sample Incident Title
New Status: resolved
Severity: SEV-2
Services: web-service, database-service
Commander: John Doe

For more details, visit the incident page."#
                        .to_string(),
                )
                .name("Example-Incident".to_string())
                .subject("Incident Update: Sample Incident Title - resolved".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentNotificationTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_notification_template(
            notification_template_data_id.clone(),
            body,
            UpdateIncidentNotificationTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
