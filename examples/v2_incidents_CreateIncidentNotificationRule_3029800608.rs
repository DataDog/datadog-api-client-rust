// Create incident notification rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::CreateIncidentNotificationRuleRequest;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleConditionsItems;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateAttributesVisibility;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateData;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateDataRelationships;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleType;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentTypeData;

#[tokio::main]
async fn main() {
    // there is a valid "incident_type" in the system
    let incident_type_data_id = std::env::var("INCIDENT_TYPE_DATA_ID").unwrap();
    let body = CreateIncidentNotificationRuleRequest::new(
        IncidentNotificationRuleCreateData::new(
            IncidentNotificationRuleCreateAttributes::new(
                vec![IncidentNotificationRuleConditionsItems::new(
                    "severity".to_string(),
                    vec!["SEV-1".to_string(), "SEV-2".to_string()],
                )],
                vec!["@test-email@company.com".to_string()],
                "incident_created_trigger".to_string(),
            )
            .enabled(true)
            .visibility(IncidentNotificationRuleCreateAttributesVisibility::ORGANIZATION),
            IncidentNotificationRuleType::INCIDENT_NOTIFICATION_RULES,
        )
        .relationships(
            IncidentNotificationRuleCreateDataRelationships::new().incident_type(
                RelationshipToIncidentType::new(RelationshipToIncidentTypeData::new(
                    incident_type_data_id.clone(),
                    IncidentTypeType::INCIDENT_TYPES,
                )),
            ),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentNotificationRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_notification_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
