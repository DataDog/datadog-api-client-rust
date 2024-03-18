// Create an API test with multi subtype returns "OK - Returns the created test
// details." response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

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
                        serde_json::Value::from(200),
                        SyntheticsAssertionType::STATUS_CODE,
                    ),
                ))],
                "request is sent".to_string(),
                SyntheticsTestRequest::new()
                    .method("GET".to_string())
                    .timeout(10 as f64)
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
                    .interval(1000 as f64),
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
                    .interval(1000 as f64),
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
