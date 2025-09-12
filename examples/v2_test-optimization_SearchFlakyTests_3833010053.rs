// Search flaky tests returns "OK" response with cursor pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::SearchFlakyTestsOptionalParams;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::FlakyTestsSearchFilter;
use datadog_api_client::datadogV2::model::FlakyTestsSearchPageOptions;
use datadog_api_client::datadogV2::model::FlakyTestsSearchRequest;
use datadog_api_client::datadogV2::model::FlakyTestsSearchRequestAttributes;
use datadog_api_client::datadogV2::model::FlakyTestsSearchRequestData;
use datadog_api_client::datadogV2::model::FlakyTestsSearchRequestDataType;
use datadog_api_client::datadogV2::model::FlakyTestsSearchSort;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let body = FlakyTestsSearchRequest::new().data(
        FlakyTestsSearchRequestData::new()
            .attributes(
                FlakyTestsSearchRequestAttributes::new()
                    .filter(FlakyTestsSearchFilter::new().query("*".to_string()))
                    .page(FlakyTestsSearchPageOptions::new().limit(2))
                    .sort(FlakyTestsSearchSort::FQN_ASCENDING),
            )
            .type_(FlakyTestsSearchRequestDataType::SEARCH_FLAKY_TESTS_REQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = TestOptimizationAPI::with_config(configuration);
    let response = api
        .search_flaky_tests_with_pagination(SearchFlakyTestsOptionalParams::default().body(body));
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}
