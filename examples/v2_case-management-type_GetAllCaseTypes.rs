// Get all case types returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management_type::CaseManagementTypeAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CaseManagementTypeAPI::with_config(configuration);
    let resp = api.get_all_case_types().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
