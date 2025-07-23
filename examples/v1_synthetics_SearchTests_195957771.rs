// Search Synthetic tests with boolean query parameters
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SearchTestsOptionalParams;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .search_tests(
            SearchTestsOptionalParams::default()
                .text("tag:value".to_string())
                .include_full_config(true)
                .search_suites(true)
                .facets_only(true)
                .start(10)
                .count(5)
                .sort("name,desc".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
