// Update incident notification rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentNotificationRuleOptionalParams;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleConditionsItems;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateAttributes;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateAttributesVisibility;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleCreateDataRelationships;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleType;
use datadog_api_client::datadogV2::model::IncidentNotificationRuleUpdateData;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::PutIncidentNotificationRuleRequest;
use datadog_api_client::datadogV2::model::RelationshipToIncidentType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentTypeData;

#[tokio::main]
async fn main() {
    // there is a valid "notification_rule" in the system
    let notification_rule_data_id =
        uuid::Uuid::parse_str(&std::env::var("NOTIFICATION_RULE_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "incident_type" in the system
    let incident_type_data_id = std::env::var("INCIDENT_TYPE_DATA_ID").unwrap();
    let body = PutIncidentNotificationRuleRequest::new(
        IncidentNotificationRuleUpdateData::new(
            IncidentNotificationRuleCreateAttributes::new(
                vec![IncidentNotificationRuleConditionsItems::new(
                    "severity".to_string(),
                    vec!["SEV-1".to_string()],
                )],
                vec!["@updated-team-email@company.com".to_string()],
                "incident_modified_trigger".to_string(),
            )
            .enabled(false)
            .visibility(IncidentNotificationRuleCreateAttributesVisibility::PRIVATE),
            notification_rule_data_id.clone(),
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
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentNotificationRule", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_notification_rule(
            notification_rule_data_id.clone(),
            body,
            UpdateIncidentNotificationRuleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
