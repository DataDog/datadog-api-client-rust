// Delete Test Optimization service settings returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::TestOptimizationDeleteServiceSettingsRequest;
use datadog_api_client::datadogV2::model::TestOptimizationDeleteServiceSettingsRequestAttributes;
use datadog_api_client::datadogV2::model::TestOptimizationDeleteServiceSettingsRequestData;
use datadog_api_client::datadogV2::model::TestOptimizationDeleteServiceSettingsRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        TestOptimizationDeleteServiceSettingsRequest::new(
            TestOptimizationDeleteServiceSettingsRequestData::new(
                TestOptimizationDeleteServiceSettingsRequestAttributes::new(
                    "github.com/datadog/shopist".to_string(),
                    "shopist".to_string(),
                ).env("prod".to_string()),
                TestOptimizationDeleteServiceSettingsRequestDataType
                ::TEST_OPTIMIZATION_DELETE_SERVICE_SETTINGS_REQUEST,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api.delete_test_optimization_service_settings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
