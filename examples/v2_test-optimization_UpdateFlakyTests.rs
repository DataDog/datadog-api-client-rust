// Update flaky test states returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::UpdateFlakyTestsRequest;
use datadog_api_client::datadogV2::model::UpdateFlakyTestsRequestAttributes;
use datadog_api_client::datadogV2::model::UpdateFlakyTestsRequestData;
use datadog_api_client::datadogV2::model::UpdateFlakyTestsRequestDataType;
use datadog_api_client::datadogV2::model::UpdateFlakyTestsRequestTest;
use datadog_api_client::datadogV2::model::UpdateFlakyTestsRequestTestNewState;

#[tokio::main]
async fn main() {
    let body = UpdateFlakyTestsRequest::new(UpdateFlakyTestsRequestData::new(
        UpdateFlakyTestsRequestAttributes::new(vec![UpdateFlakyTestsRequestTest::new(
            "4eb1887a8adb1847".to_string(),
            UpdateFlakyTestsRequestTestNewState::ACTIVE,
        )]),
        UpdateFlakyTestsRequestDataType::UPDATE_FLAKY_TEST_STATE_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateFlakyTests", true);
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api.update_flaky_tests(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
