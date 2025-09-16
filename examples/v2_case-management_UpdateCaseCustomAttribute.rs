// Update case custom attribute returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseResourceType;
use datadog_api_client::datadogV2::model::CaseUpdateCustomAttribute;
use datadog_api_client::datadogV2::model::CaseUpdateCustomAttributeRequest;
use datadog_api_client::datadogV2::model::CustomAttributeType;
use datadog_api_client::datadogV2::model::CustomAttributeValue;
use datadog_api_client::datadogV2::model::CustomAttributeValuesUnion;

#[tokio::main]
async fn main() {
    // there is a valid "case" with a custom "case_type" in the system
    let case_with_type_id = std::env::var("CASE_WITH_TYPE_ID").unwrap();

    // there is a valid "custom_attribute" in the system
    let custom_attribute_attributes_key = std::env::var("CUSTOM_ATTRIBUTE_ATTRIBUTES_KEY").unwrap();
    let body = CaseUpdateCustomAttributeRequest::new(CaseUpdateCustomAttribute::new(
        CustomAttributeValue::new(
            true,
            CustomAttributeType::TEXT,
            CustomAttributeValuesUnion::CustomAttributeMultiStringValue(vec![
                "Abba".to_string(),
                "The Cure".to_string(),
            ]),
        ),
        CaseResourceType::CASE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_case_custom_attribute(
            case_with_type_id.clone(),
            custom_attribute_attributes_key.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
