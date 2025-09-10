// Create an incident notification rule returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::CreateIncidentNotificationRuleRequest;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleConditionsItems;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateAttributesVisibility;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateData;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateDataRelationships;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleType;
use datadog_api_client::datadogV2::model::IncidentNotificationTemplateType;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentNotificationTemplate;
use datadog_api_client::datadogV2::model::RelationshipToIncidentNotificationTemplateData;
use datadog_api_client::datadogV2::model::RelationshipToIncidentType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentTypeData;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateIncidentNotificationRuleRequest::new(
        IncidentNotificationRuleCreateData::new(
            IncidentNotificationRuleCreateAttributes::new(
                vec![IncidentNotificationRuleConditionsItems::new(
                    "severity".to_string(),
                    vec!["SEV-1".to_string(), "SEV-2".to_string()],
                )],
                vec![
                    "@team-email@company.com".to_string(),
                    "@slack-channel".to_string(),
                ],
                "incident_created_trigger".to_string(),
            )
            .enabled(true)
            .renotify_on(vec!["status".to_string(), "severity".to_string()])
            .visibility(IncidentNotificationRuleCreateAttributesVisibility::ORGANIZATION),
            IncidentNotificationRuleType::INCIDENT_NOTIFICATION_RULES,
        )
        .relationships(
            IncidentNotificationRuleCreateDataRelationships::new()
                .incident_type(RelationshipToIncidentType::new(
                    RelationshipToIncidentTypeData::new(
                        "00000000-0000-0000-0000-000000000000".to_string(),
                        IncidentTypeType::INCIDENT_TYPES,
                    ),
                ))
                .notification_template(RelationshipToIncidentNotificationTemplate::new(
                    RelationshipToIncidentNotificationTemplateData::new(
                        Uuid::parse_str("00000000-0000-0000-0000-000000000001")
                            .expect("invalid UUID"),
                        IncidentNotificationTemplateType::NOTIFICATION_TEMPLATES,
                    ),
                )),
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
