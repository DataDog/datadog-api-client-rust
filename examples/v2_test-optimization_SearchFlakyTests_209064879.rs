// Search flaky tests returns "OK" response with history
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
                        .page(FlakyTestsSearchPageOptions::new().limit(10))
                        .sort(FlakyTestsSearchSort::FQN_ASCENDING),
                )
                .type_(FlakyTestsSearchRequestDataType::SEARCH_FLAKY_TESTS_REQUEST),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SearchFlakyTests", true);
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api
        .search_flaky_tests(SearchFlakyTestsOptionalParams::default().body(body))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
