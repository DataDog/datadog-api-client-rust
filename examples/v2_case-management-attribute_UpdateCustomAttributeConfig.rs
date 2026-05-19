// Update custom attribute config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_attribute::CaseManagementAttributeAPI;
use datadog_api_client::datadogV2::model::CustomAttributeConfigResourceType;
use datadog_api_client::datadogV2::model::CustomAttributeConfigUpdate;
use datadog_api_client::datadogV2::model::CustomAttributeConfigUpdateAttributes;
use datadog_api_client::datadogV2::model::CustomAttributeConfigUpdateRequest;
use datadog_api_client::datadogV2::model::CustomAttributeSelectOption;
use datadog_api_client::datadogV2::model::CustomAttributeType;
use datadog_api_client::datadogV2::model::CustomAttributeTypeData;

#[tokio::main]
async fn main() {
    let body = CustomAttributeConfigUpdateRequest::new(
        CustomAttributeConfigUpdate::new(CustomAttributeConfigResourceType::CUSTOM_ATTRIBUTE)
            .attributes(
                CustomAttributeConfigUpdateAttributes::new()
                    .description("Updated description.".to_string())
                    .display_name("AWS Region".to_string())
                    .type_(CustomAttributeType::NUMBER)
                    .type_data(CustomAttributeTypeData::new().options(vec![
                        CustomAttributeSelectOption::new("us-east-1".to_string()),
                    ])),
            ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateCustomAttributeConfig", true);
    let api = CaseManagementAttributeAPI::with_config(configuration);
    let resp = api
        .update_custom_attribute_config(
            "case_type_id".to_string(),
            "custom_attribute_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
