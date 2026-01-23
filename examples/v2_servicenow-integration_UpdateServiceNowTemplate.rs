// Update ServiceNow template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_now_integration::ServiceNowIntegrationAPI;
use datadog_api_client::datadogV2::model::ServiceNowTemplateType;
use datadog_api_client::datadogV2::model::ServiceNowTemplateUpdateRequest;
use datadog_api_client::datadogV2::model::ServiceNowTemplateUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::ServiceNowTemplateUpdateRequestData;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ServiceNowTemplateUpdateRequest::new(ServiceNowTemplateUpdateRequestData::new(
        ServiceNowTemplateUpdateRequestAttributes::new(
            "incident-template-updated".to_string(),
            Uuid::parse_str("65b3341b-0680-47f9-a6d4-134db45c603e").expect("invalid UUID"),
            "incident".to_string(),
        )
        .assignment_group_id(
            Uuid::parse_str("65b3341b-0680-47f9-a6d4-134db45c603e").expect("invalid UUID"),
        )
        .business_service_id(
            Uuid::parse_str("65b3341b-0680-47f9-a6d4-134db45c603e").expect("invalid UUID"),
        )
        .fields_mapping(BTreeMap::from([
            ("category".to_string(), "hardware".to_string()),
            ("priority".to_string(), "2".to_string()),
        ]))
        .user_id(Uuid::parse_str("65b3341b-0680-47f9-a6d4-134db45c603e").expect("invalid UUID")),
        ServiceNowTemplateType::SERVICENOW_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateServiceNowTemplate", true);
    let api = ServiceNowIntegrationAPI::with_config(configuration);
    let resp = api
        .update_service_now_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
