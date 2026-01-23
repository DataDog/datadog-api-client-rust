// Create ServiceNow template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_now_integration::ServiceNowIntegrationAPI;
use datadog_api_client::datadogV2::model::ServiceNowTemplateCreateRequest;
use datadog_api_client::datadogV2::model::ServiceNowTemplateCreateRequestAttributes;
use datadog_api_client::datadogV2::model::ServiceNowTemplateCreateRequestData;
use datadog_api_client::datadogV2::model::ServiceNowTemplateType;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ServiceNowTemplateCreateRequest::new(ServiceNowTemplateCreateRequestData::new(
        ServiceNowTemplateCreateRequestAttributes::new(
            "incident-template".to_string(),
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
            ("category".to_string(), "software".to_string()),
            ("priority".to_string(), "1".to_string()),
        ]))
        .user_id(Uuid::parse_str("65b3341b-0680-47f9-a6d4-134db45c603e").expect("invalid UUID")),
        ServiceNowTemplateType::SERVICENOW_TEMPLATES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateServiceNowTemplate", true);
    let api = ServiceNowIntegrationAPI::with_config(configuration);
    let resp = api.create_service_now_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
