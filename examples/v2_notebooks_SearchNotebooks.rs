// Search notebooks returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_notebooks::NotebooksAPI;
use datadog_api_client::datadogV2::api_notebooks::SearchNotebooksOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchNotebooks", true);
    let api = NotebooksAPI::with_config(configuration);
    let resp = api
        .search_notebooks(SearchNotebooksOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
