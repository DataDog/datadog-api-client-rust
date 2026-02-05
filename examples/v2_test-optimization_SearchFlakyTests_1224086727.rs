// Search flaky tests returns "OK" response with pagination
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
    let body =
        FlakyTestsSearchRequest
        ::new().data(
            FlakyTestsSearchRequestData::new()
                .attributes(
                    FlakyTestsSearchRequestAttributes::new()
                        .filter(
                            FlakyTestsSearchFilter
                            ::new().query(
                                r#"flaky_test_state:active @git.repository.id_v2:"github.com/datadog/shopist""#.to_string(),
                            ),
                        )
                        .include_history(true)
                        .page(
                            FlakyTestsSearchPageOptions::new()
                                .cursor(
                                    "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ==".to_string(),
                                )
                                .limit(25),
                        )
                        .sort(FlakyTestsSearchSort::FAILURE_RATE_ASCENDING),
                )
                .type_(FlakyTestsSearchRequestDataType::SEARCH_FLAKY_TESTS_REQUEST),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchFlakyTests", true);
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
