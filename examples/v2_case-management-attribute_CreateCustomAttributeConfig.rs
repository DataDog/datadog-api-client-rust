// Create custom attribute config for a case type returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_attribute::CaseManagementAttributeAPI;
use datadog_api_client::datadogV2::model::CustomAttributeConfigAttributesCreate;
use datadog_api_client::datadogV2::model::CustomAttributeConfigCreate;
use datadog_api_client::datadogV2::model::CustomAttributeConfigCreateRequest;
use datadog_api_client::datadogV2::model::CustomAttributeConfigResourceType;
use datadog_api_client::datadogV2::model::CustomAttributeType;

#[tokio::main]
async fn main() {
    // there is a valid "case_type" in the system
    let case_type_id = std::env::var("CASE_TYPE_ID").unwrap();
    let body = CustomAttributeConfigCreateRequest::new(CustomAttributeConfigCreate::new(
        CustomAttributeConfigAttributesCreate::new(
            "AWS Region ".to_string(),
            true,
            "region_d9fe56bc9274fbb6".to_string(),
            CustomAttributeType::NUMBER,
        ),
        CustomAttributeConfigResourceType::CUSTOM_ATTRIBUTE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAttributeAPI::with_config(configuration);
    let resp = api
        .create_custom_attribute_config(case_type_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
