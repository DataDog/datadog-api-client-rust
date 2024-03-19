// Create an API test with multi subtype returns "OK - Returns the created test
// details." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsAPIStep;
use datadog_api_client::datadogV1::model::SyntheticsAPIStepSubtype;
use datadog_api_client::datadogV1::model::SyntheticsAPITest;
use datadog_api_client::datadogV1::model::SyntheticsAPITestConfig;
use datadog_api_client::datadogV1::model::SyntheticsAPITestType;
use datadog_api_client::datadogV1::model::SyntheticsAssertion;
use datadog_api_client::datadogV1::model::SyntheticsAssertionOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionType;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariable;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariableType;
use datadog_api_client::datadogV1::model::SyntheticsGlobalVariableParseTestOptionsType;
use datadog_api_client::datadogV1::model::SyntheticsGlobalVariableParserType;
use datadog_api_client::datadogV1::model::SyntheticsParsingOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestDetailsSubType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;
use datadog_api_client::datadogV1::model::SyntheticsVariableParser;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let body = SyntheticsAPITest::new(
        SyntheticsAPITestConfig::new()
            .config_variables(vec![SyntheticsConfigVariable::new(
                "PROPERTY".to_string(),
                SyntheticsConfigVariableType::TEXT,
            )
            .example("content-type".to_string())
            .pattern("content-type".to_string())])
            .steps(vec![SyntheticsAPIStep::new(
                vec![SyntheticsAssertion::SyntheticsAssertionTarget(Box::new(
                    SyntheticsAssertionTarget::new(
                        SyntheticsAssertionOperator::IS,
                        Value::from(200),
                        SyntheticsAssertionType::STATUS_CODE,
                    ),
                ))],
                "request is sent".to_string(),
                SyntheticsTestRequest::new()
                    .method("GET".to_string())
                    .timeout(10.0 as f64)
                    .url("https://datadoghq.com".to_string()),
                SyntheticsAPIStepSubtype::HTTP,
            )
            .allow_failure(true)
            .extracted_values(vec![SyntheticsParsingOptions::new()
                .field("server".to_string())
                .name("EXTRACTED_VALUE".to_string())
                .parser(SyntheticsVariableParser::new(
                    SyntheticsGlobalVariableParserType::RAW,
                ))
                .secure(true)
                .type_(SyntheticsGlobalVariableParseTestOptionsType::HTTP_HEADER)])
            .is_critical(true)
            .retry(
                SyntheticsTestOptionsRetry::new()
                    .count(5)
                    .interval(1000.0 as f64),
            )]),
        vec!["aws:us-east-2".to_string()],
        "BDD test payload: synthetics_api_test_multi_step_payload.json".to_string(),
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
                    .interval(1000.0 as f64),
            )
            .tick_every(60),
        SyntheticsAPITestType::API,
    )
    .subtype(SyntheticsTestDetailsSubType::MULTI)
    .tags(vec!["testing:api".to_string()]);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_api_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
