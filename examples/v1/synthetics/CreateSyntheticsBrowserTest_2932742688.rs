// Create a browser test returns "OK - Returns saved rumSettings." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        SyntheticsBrowserTest::new(
            SyntheticsBrowserTestConfig::new(
                vec![],
                SyntheticsTestRequest::new()
                    .certificate_domains(vec!["https://datadoghq.com".to_string()])
                    .method("GET".to_string())
                    .url("https://datadoghq.com".to_string()),
            )
                .config_variables(
                    vec![
                        SyntheticsConfigVariable::new("PROPERTY".to_string(), SyntheticsConfigVariableType::TEXT)
                            .example("content-type".to_string())
                            .pattern("content-type".to_string())
                    ],
                )
                .set_cookie("name:test".to_string()),
            vec!["aws:us-east-2".to_string()],
            "Test message".to_string(),
            "Example-Synthetic".to_string(),
            SyntheticsTestOptions::new()
                .accept_self_signed(false)
                .allow_insecure(true)
                .ci(SyntheticsTestCiOptions::new().execution_rule(SyntheticsTestExecutionRule::SKIPPED))
                .device_ids(vec![SyntheticsDeviceID::TABLET])
                .disable_cors(true)
                .disable_csp(true)
                .follow_redirects(true)
                .ignore_server_certificate_error(true)
                .initial_navigation_timeout(200)
                .min_failure_duration(10)
                .min_location_failed(1)
                .no_screenshot(true)
                .retry(SyntheticsTestOptionsRetry::new().count(2).interval(10))
                .rum_settings(
                    SyntheticsBrowserTestRumSettings::new(true)
                        .application_id("mockApplicationId".to_string())
                        .client_token_id(12345),
                )
                .tick_every(300),
            SyntheticsBrowserTestType::BROWSER,
        )
            .steps(
                vec![
                    SyntheticsStep::new()
                        .allow_failure(false)
                        .is_critical(true)
                        .name("Refresh page".to_string())
                        .params(std::collections::BTreeMap::new())
                        .type_(SyntheticsStepType::REFRESH)
                ],
            )
            .tags(vec!["testing:browser".to_string()]);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_browser_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
