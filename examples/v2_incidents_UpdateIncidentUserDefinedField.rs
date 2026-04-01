// Update an incident user-defined field returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::UpdateIncidentUserDefinedFieldOptionalParams;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldAttributesUpdateRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCategory;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldCollected;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldType;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldUpdateData;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldUpdateRequest;
use datadog_api_client::datadogV2::model::IncidentUserDefinedFieldValidValue;

#[tokio::main]
async fn main() {
    let body = IncidentUserDefinedFieldUpdateRequest::new(IncidentUserDefinedFieldUpdateData::new(
        IncidentUserDefinedFieldAttributesUpdateRequest::new()
            .category(Some(IncidentUserDefinedFieldCategory::WHAT_HAPPENED))
            .collected(Some(IncidentUserDefinedFieldCollected::ACTIVE))
            .default_value(Some("critical".to_string()))
            .display_name("Root Cause".to_string())
            .ordinal(Some("1.5".to_string()))
            .required(Some(false))
            .valid_values(Some(vec![IncidentUserDefinedFieldValidValue::new(
                "Critical".to_string(),
                "critical".to_string(),
            )
            .description("A critical severity incident.".to_string())
            .short_description("Critical".to_string())])),
        "00000000-0000-0000-0000-000000000000".to_string(),
        IncidentUserDefinedFieldType::USER_DEFINED_FIELD,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIncidentUserDefinedField", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .update_incident_user_defined_field(
            "00000000-0000-0000-0000-000000000000".to_string(),
            body,
            UpdateIncidentUserDefinedFieldOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
