// Delete custom attributes config returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_attribute::CaseManagementAttributeAPI;

#[tokio::main]
async fn main() {
    // there is a valid "case_type" in the system
    let case_type_id = std::env::var("CASE_TYPE_ID").unwrap();

    // there is a valid "custom_attribute" in the system
    let custom_attribute_id = std::env::var("CUSTOM_ATTRIBUTE_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAttributeAPI::with_config(configuration);
    let resp = api
        .delete_custom_attribute_config(case_type_id.clone(), custom_attribute_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
