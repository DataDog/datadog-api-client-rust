// Create an incident user-defined field returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::CreateIncidentUserDefinedFieldOptionalParams;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentTypeType;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldAttributesCreateRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCategory;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCollected;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCreateData;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCreateRelationships;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCreateRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldFieldType;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldType;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldValidValue;
use datadog_api_client::datadogV2::model::RelationshipToIncidentType;
use datadog_api_client::datadogV2::model::RelationshipToIncidentTypeData;

#[tokio::main]
async fn main() {
    let body = IncidentUserDefinedFieldCreateRequest::new(IncidentUserDefinedFieldCreateData::new(
        IncidentUserDefinedFieldAttributesCreateRequest::new(
            "root_cause".to_string(),
            IncidentUserDefinedFieldFieldType::TEXTBOX,
        )
        .category(Some(IncidentUserDefinedFieldCategory::WHAT_HAPPENED))
        .collected(Some(IncidentUserDefinedFieldCollected::ACTIVE))
        .default_value(Some("critical".to_string()))
        .display_name("Root Cause".to_string())
        .ordinal(Some("1.5".to_string()))
        .required(false)
        .tag_key(Some("datacenter".to_string()))
        .valid_values(vec![IncidentUserDefinedFieldValidValue::new(
            "A critical severity incident.".to_string(),
            "Critical".to_string(),
            "critical".to_string(),
        )
        .short_description("Critical".to_string())]),
        IncidentUserDefinedFieldCreateRelationships::new(RelationshipToIncidentType::new(
            RelationshipToIncidentTypeData::new(
                "00000000-0000-0000-0000-000000000000".to_string(),
                IncidentTypeType::INCIDENT_TYPES,
            ),
        )),
        IncidentUserDefinedFieldType::USER_DEFINED_FIELD,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentUserDefinedField", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_user_defined_field(
            body,
            CreateIncidentUserDefinedFieldOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
