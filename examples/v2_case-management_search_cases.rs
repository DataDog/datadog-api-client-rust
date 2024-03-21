// Search cases returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::api::api_case_management::SearchCasesOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.search_cases(SearchCasesOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
