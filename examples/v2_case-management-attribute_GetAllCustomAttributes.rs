// Get all custom attributes returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_attribute::CaseManagementAttributeAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAttributeAPI::with_config(configuration);
    let resp = api.get_all_custom_attributes().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
