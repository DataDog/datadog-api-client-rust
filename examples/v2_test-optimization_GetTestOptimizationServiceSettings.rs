// Get Test Optimization service settings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::TestOptimizationGetServiceSettingsRequest;
use datadog_api_client::datadogV2::model::TestOptimizationGetServiceSettingsRequestAttributes;
use datadog_api_client::datadogV2::model::TestOptimizationGetServiceSettingsRequestData;
use datadog_api_client::datadogV2::model::TestOptimizationGetServiceSettingsRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        TestOptimizationGetServiceSettingsRequest::new(
            TestOptimizationGetServiceSettingsRequestData::new(
                TestOptimizationGetServiceSettingsRequestAttributes::new(
                    "github.com/datadog/shopist".to_string(),
                    "shopist".to_string(),
                ).env("prod".to_string()),
                TestOptimizationGetServiceSettingsRequestDataType::TEST_OPTIMIZATION_GET_SERVICE_SETTINGS_REQUEST,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetTestOptimizationServiceSettings", true);
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api.get_test_optimization_service_settings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
