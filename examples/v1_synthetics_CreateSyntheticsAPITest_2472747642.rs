// Create an API test with WEBSOCKET subtype returns "OK - Returns the created
// test details." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsAPITest;
use datadog_api_client::datadogV1::model::SyntheticsAPITestConfig;
use datadog_api_client::datadogV1::model::SyntheticsAPITestType;
use datadog_api_client::datadogV1::model::SyntheticsAssertion;
use datadog_api_client::datadogV1::model::SyntheticsAssertionOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTargetValue;
use datadog_api_client::datadogV1::model::SyntheticsAssertionType;
use datadog_api_client::datadogV1::model::SyntheticsTestDetailsSubType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;

#[tokio::main]
async fn main() {
    let body = SyntheticsAPITest::new(
        SyntheticsAPITestConfig::new()
            .assertions(vec![
                SyntheticsAssertion::SyntheticsAssertionTarget(Box::new(
                    SyntheticsAssertionTarget::new(
                        SyntheticsAssertionOperator::IS,
                        SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueString(
                            "message".to_string(),
                        ),
                        SyntheticsAssertionType::RECEIVED_MESSAGE,
                    ),
                )),
                SyntheticsAssertion::SyntheticsAssertionTarget(Box::new(
                    SyntheticsAssertionTarget::new(
                        SyntheticsAssertionOperator::LESS_THAN,
                        SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                            2000.0 as f64,
                        ),
                        SyntheticsAssertionType::RESPONSE_TIME,
                    ),
                )),
            ])
            .config_variables(vec![])
            .request(
                SyntheticsTestRequest::new()
                    .message("message".to_string())
                    .url("ws://datadoghq.com".to_string()),
            ),
        vec!["aws:us-east-2".to_string()],
        "BDD test payload: synthetics_api_test_websocket_payload.json".to_string(),
        "Example-Synthetic".to_string(),
        SyntheticsTestOptions::new()
            .accept_self_signed(false)
            .allow_insecure(true)
            .follow_redirects(true)
            .min_failure_duration(10)
            .min_location_failed(1)
            .monitor_name("Example-Synthetic".to_string())
            .monitor_priority(5)
            .retry(
                SyntheticsTestOptionsRetry::new()
                    .count(3)
                    .interval(10.0 as f64),
            )
            .tick_every(60),
        SyntheticsAPITestType::API,
    )
    .subtype(SyntheticsTestDetailsSubType::WEBSOCKET)
    .tags(vec!["testing:api".to_string()]);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_api_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
