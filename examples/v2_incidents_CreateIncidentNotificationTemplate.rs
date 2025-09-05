// Create incident notification template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::CreateIncidentNotificationTemplateRequest;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateCreateData;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateCreateDataRelationships;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateType;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentTypeData;

#[tokio::main]
async fn main() {
    // there is a valid "incident_type" in the system
    let incident_type_data_id = std::env::var("INCIDENT_TYPE_DATA_ID").unwrap();
    let body = CreateIncidentNotificationTemplateRequest::new(
        IncidentNotificationTemplateCreateData::new(
            IncidentNotificationTemplateCreateAttributes::new(
                "alert".to_string(),
                r#"An incident has been declared.

Title: Sample Incident Title
Severity: SEV-2
Affected Services: web-service, database-service
Status: active

Please join the incident channel for updates."#
                    .to_string(),
                "Example-Incident".to_string(),
                "SEV-2 Incident: Sample Incident Title".to_string(),
            ),
            IncidentNotificationTemplateType::NOTIFICATION_TEMPLATES,
        )
        .relationships(
            IncidentNotificationTemplateCreateDataRelationships::new().incident_type(
                RelationshipToIncidentType::new(RelationshipToIncidentTypeData::new(
                    incident_type_data_id.clone(),
                    IncidentTypeType::INCIDENT_TYPES,
                )),
            ),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentNotificationTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_notification_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
