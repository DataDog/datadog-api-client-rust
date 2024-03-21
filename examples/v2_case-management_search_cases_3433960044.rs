// Search cases returns "OK" response with pagination
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::api::api_case_management::SearchCasesOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let response = api.search_cases_with_pagination(SearchCasesOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
