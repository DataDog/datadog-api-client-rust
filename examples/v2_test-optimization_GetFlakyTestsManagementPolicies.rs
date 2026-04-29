// Get Flaky Tests Management policies returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesGetRequest;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesGetRequestAttributes;
use datadog_api_client::datadogV2::model::TestOptimizationFlakyTestsManagementPoliciesGetRequestData;
use datadog_api_client::datadogV2::model::TestOptimizationGetFlakyTestsManagementPoliciesRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        TestOptimizationFlakyTestsManagementPoliciesGetRequest::new(
            TestOptimizationFlakyTestsManagementPoliciesGetRequestData::new(
                TestOptimizationFlakyTestsManagementPoliciesGetRequestAttributes::new(
                    "github.com/datadog/shopist".to_string(),
                ),
                TestOptimizationGetFlakyTestsManagementPoliciesRequestDataType
                ::TEST_OPTIMIZATION_GET_FLAKY_TESTS_MANAGEMENT_POLICIES_REQUEST,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api.get_flaky_tests_management_policies(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
