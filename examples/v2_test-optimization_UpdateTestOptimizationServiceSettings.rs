// Update Test Optimization service settings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_test_optimization::TestOptimizationAPI;
use datadog_api_client::datadogV2::model::TestOptimizationUpdateServiceSettingsRequest;
use datadog_api_client::datadogV2::model::TestOptimizationUpdateServiceSettingsRequestAttributes;
use datadog_api_client::datadogV2::model::TestOptimizationUpdateServiceSettingsRequestData;
use datadog_api_client::datadogV2::model::TestOptimizationUpdateServiceSettingsRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        TestOptimizationUpdateServiceSettingsRequest::new(
            TestOptimizationUpdateServiceSettingsRequestData::new(
                TestOptimizationUpdateServiceSettingsRequestAttributes::new(
                    "github.com/datadog/shopist".to_string(),
                    "shopist".to_string(),
                )
                    .auto_test_retries_enabled(false)
                    .code_coverage_enabled(false)
                    .early_flake_detection_enabled(false)
                    .env("prod".to_string())
                    .failed_test_replay_enabled(false)
                    .pr_comments_enabled(true)
                    .test_impact_analysis_enabled(false),
                TestOptimizationUpdateServiceSettingsRequestDataType
                ::TEST_OPTIMIZATION_UPDATE_SERVICE_SETTINGS_REQUEST,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = TestOptimizationAPI::with_config(configuration);
    let resp = api.update_test_optimization_service_settings(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
