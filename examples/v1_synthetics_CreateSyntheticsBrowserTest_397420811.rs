// Create a browser test with advanced scheduling options returns "OK - Returns
// the created test details." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsBrowserTest;
use datadog_api_client::datadogV1::model::SyntheticsBrowserTestConfig;
use datadog_api_client::datadogV1::model::SyntheticsBrowserTestType;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariable;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariableType;
use datadog_api_client::datadogV1::model::SyntheticsStep;
use datadog_api_client::datadogV1::model::SyntheticsStepType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsScheduling;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsSchedulingTimeframe;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;
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
            .device_ids(vec!["tablet".to_string()])
            .disable_cors(true)
            .follow_redirects(true)
            .min_failure_duration(10)
            .min_location_failed(1)
            .no_screenshot(true)
            .retry(
                SyntheticsTestOptionsRetry::new()
                    .count(2)
                    .interval(10.0 as f64),
            )
            .scheduling(SyntheticsTestOptionsScheduling::new(
                vec![
                    SyntheticsTestOptionsSchedulingTimeframe::new(
                        1,
                        "07:00".to_string(),
                        "16:00".to_string(),
                    ),
                    SyntheticsTestOptionsSchedulingTimeframe::new(
                        3,
                        "07:00".to_string(),
                        "16:00".to_string(),
                    ),
                ],
                "America/New_York".to_string(),
            ))
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
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_browser_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
