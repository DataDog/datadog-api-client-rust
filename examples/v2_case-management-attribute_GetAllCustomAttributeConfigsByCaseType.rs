// Get all custom attributes config of case type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_attribute::CaseManagementAttributeAPI;

#[tokio::main]
async fn main() {
    // there is a valid "case_type" in the system
    let case_type_id = std::env::var("CASE_TYPE_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAttributeAPI::with_config(configuration);
    let resp = api
        .get_all_custom_attribute_configs_by_case_type(case_type_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
