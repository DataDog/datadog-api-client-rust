// Create a browser test with advanced scheduling options returns "OK - Returns
// the created test details." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = SyntheticsBrowserTest::new(
        SyntheticsBrowserTestConfig::new(
            vec![],
            SyntheticsTestRequest::new()
                .method("GET".to_string())
                .url("https://datadoghq.com".to_string()),
        )
        .config_variables(vec![SyntheticsConfigVariable::new(
            "PROPERTY".to_string(),
            SyntheticsConfigVariableType::TEXT,
        )
        .example("content-type".to_string())
        .pattern("content-type".to_string())])
        .set_cookie("name:test".to_string()),
        vec!["aws:us-east-2".to_string()],
        "Test message".to_string(),
        "Example-Synthetic".to_string(),
        SyntheticsTestOptions::new()
            .accept_self_signed(false)
            .allow_insecure(true)
            .device_ids(vec![SyntheticsDeviceID::TABLET])
            .disable_cors(true)
            .follow_redirects(true)
            .min_failure_duration(10)
            .min_location_failed(1)
            .no_screenshot(true)
            .retry(
                SyntheticsTestOptionsRetry::new()
                    .count(2)
                    .interval(10 as f64),
            )
            .scheduling(
                SyntheticsTestOptionsScheduling::new()
                    .timeframes(vec![
                        SyntheticsTestOptionsSchedulingTimeframe::new()
                            .day(1)
                            .from("07:00".to_string())
                            .to("16:00".to_string()),
                        SyntheticsTestOptionsSchedulingTimeframe::new()
                            .day(3)
                            .from("07:00".to_string())
                            .to("16:00".to_string()),
                    ])
                    .timezone("America/New_York".to_string()),
            )
            .tick_every(300),
        SyntheticsBrowserTestType::BROWSER,
    )
    .steps(vec![SyntheticsStep::new()
        .allow_failure(false)
        .is_critical(true)
        .name("Refresh page".to_string())
        .params(BTreeMap::new())
        .type_(SyntheticsStepType::REFRESH)])
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
