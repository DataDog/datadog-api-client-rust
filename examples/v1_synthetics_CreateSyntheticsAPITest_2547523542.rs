// Create an API test with MCP steps returns "OK - Returns the created test
// details." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsAPIStep;
use datadog_api_client::datadogV1::model::SyntheticsAPITest;
use datadog_api_client::datadogV1::model::SyntheticsAPITestConfig;
use datadog_api_client::datadogV1::model::SyntheticsAPITestStep;
use datadog_api_client::datadogV1::model::SyntheticsAPITestStepSubtype;
use datadog_api_client::datadogV1::model::SyntheticsAPITestType;
use datadog_api_client::datadogV1::model::SyntheticsAssertion;
use datadog_api_client::datadogV1::model::SyntheticsAssertionMCPRespectsSpecification;
use datadog_api_client::datadogV1::model::SyntheticsAssertionMCPRespectsSpecificationType;
use datadog_api_client::datadogV1::model::SyntheticsAssertionMCPServerCapabilitiesTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionMCPServerCapabilitiesType;
use datadog_api_client::datadogV1::model::SyntheticsAssertionOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTargetValue;
use datadog_api_client::datadogV1::model::SyntheticsAssertionType;
use datadog_api_client::datadogV1::model::SyntheticsMCPProtocolVersion;
use datadog_api_client::datadogV1::model::SyntheticsMCPServerCapability;
use datadog_api_client::datadogV1::model::SyntheticsTestCallType;
use datadog_api_client::datadogV1::model::SyntheticsTestDetailsSubType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsAPITest::new(
            SyntheticsAPITestConfig
            ::new().steps(
                vec![
                    SyntheticsAPIStep::SyntheticsAPITestStep(
                        Box::new(
                            SyntheticsAPITestStep::new(
                                vec![
                                    SyntheticsAssertion::SyntheticsAssertionTarget(
                                        Box::new(
                                            SyntheticsAssertionTarget::new(
                                                SyntheticsAssertionOperator::IS,
                                                SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                                    200.0 as f64,
                                                ),
                                                SyntheticsAssertionType::STATUS_CODE,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionMCPRespectsSpecification(
                                        Box::new(
                                            SyntheticsAssertionMCPRespectsSpecification::new(
                                                SyntheticsAssertionMCPRespectsSpecificationType
                                                ::MCP_RESPECTS_SPECIFICATION,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionMCPServerCapabilitiesTarget(
                                        Box::new(
                                            SyntheticsAssertionMCPServerCapabilitiesTarget::new(
                                                SyntheticsAssertionOperator::CONTAINS,
                                                vec![SyntheticsMCPServerCapability::TOOLS],
                                                SyntheticsAssertionMCPServerCapabilitiesType::MCP_SERVER_CAPABILITIES,
                                            ),
                                        ),
                                    )
                                ],
                                "Initialize MCP session".to_string(),
                                SyntheticsTestRequest::new()
                                    .call_type(SyntheticsTestCallType::INIT)
                                    .headers(
                                        BTreeMap::from(
                                            [
                                                ("DD-API-KEY".to_string(), "<YOUR-API-KEY>".to_string()),
                                                ("DD-APPLICATION-KEY".to_string(), "<YOUR-APP-KEY>".to_string()),
                                            ],
                                        ),
                                    )
                                    .mcp_protocol_version(SyntheticsMCPProtocolVersion::VERSION_2025_06_18)
                                    .url("https://example.org/mcp".to_string()),
                                SyntheticsAPITestStepSubtype::MCP,
                            )
                                .allow_failure(false)
                                .is_critical(true)
                                .retry(SyntheticsTestOptionsRetry::new().count(0).interval(300.0 as f64)),
                        ),
                    ),
                    SyntheticsAPIStep::SyntheticsAPITestStep(
                        Box::new(
                            SyntheticsAPITestStep::new(
                                vec![
                                    SyntheticsAssertion::SyntheticsAssertionTarget(
                                        Box::new(
                                            SyntheticsAssertionTarget::new(
                                                SyntheticsAssertionOperator::IS,
                                                SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                                    200.0 as f64,
                                                ),
                                                SyntheticsAssertionType::STATUS_CODE,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionTarget(
                                        Box::new(
                                            SyntheticsAssertionTarget::new(
                                                SyntheticsAssertionOperator::MORE_THAN,
                                                SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                                    0.0 as f64,
                                                ),
                                                SyntheticsAssertionType::MCP_TOOL_COUNT,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionTarget(
                                        Box::new(
                                            SyntheticsAssertionTarget::new(
                                                SyntheticsAssertionOperator::LESS_THAN,
                                                SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                                    64.0 as f64,
                                                ),
                                                SyntheticsAssertionType::MCP_TOOL_NAME_LENGTH,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionMCPRespectsSpecification(
                                        Box::new(
                                            SyntheticsAssertionMCPRespectsSpecification::new(
                                                SyntheticsAssertionMCPRespectsSpecificationType
                                                ::MCP_RESPECTS_SPECIFICATION,
                                            ),
                                        ),
                                    )
                                ],
                                "List MCP tools".to_string(),
                                SyntheticsTestRequest::new()
                                    .call_type(SyntheticsTestCallType::TOOL_LIST)
                                    .headers(
                                        BTreeMap::from(
                                            [
                                                ("DD-API-KEY".to_string(), "<YOUR-API-KEY>".to_string()),
                                                ("DD-APPLICATION-KEY".to_string(), "<YOUR-APP-KEY>".to_string()),
                                            ],
                                        ),
                                    )
                                    .mcp_protocol_version(SyntheticsMCPProtocolVersion::VERSION_2025_06_18)
                                    .url("https://example.org/mcp".to_string()),
                                SyntheticsAPITestStepSubtype::MCP,
                            )
                                .allow_failure(false)
                                .is_critical(true)
                                .retry(SyntheticsTestOptionsRetry::new().count(0).interval(300.0 as f64)),
                        ),
                    ),
                    SyntheticsAPIStep::SyntheticsAPITestStep(
                        Box::new(
                            SyntheticsAPITestStep::new(
                                vec![
                                    SyntheticsAssertion::SyntheticsAssertionTarget(
                                        Box::new(
                                            SyntheticsAssertionTarget::new(
                                                SyntheticsAssertionOperator::IS,
                                                SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                                    200.0 as f64,
                                                ),
                                                SyntheticsAssertionType::STATUS_CODE,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionTarget(
                                        Box::new(
                                            SyntheticsAssertionTarget::new(
                                                SyntheticsAssertionOperator::LESS_THAN,
                                                SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                                    5000.0 as f64,
                                                ),
                                                SyntheticsAssertionType::RESPONSE_TIME,
                                            ),
                                        ),
                                    ),
                                    SyntheticsAssertion::SyntheticsAssertionMCPRespectsSpecification(
                                        Box::new(
                                            SyntheticsAssertionMCPRespectsSpecification::new(
                                                SyntheticsAssertionMCPRespectsSpecificationType
                                                ::MCP_RESPECTS_SPECIFICATION,
                                            ),
                                        ),
                                    )
                                ],
                                "Call MCP search tool".to_string(),
                                SyntheticsTestRequest::new()
                                    .call_type(SyntheticsTestCallType::TOOL_CALL)
                                    .headers(
                                        BTreeMap::from(
                                            [
                                                ("DD-API-KEY".to_string(), "<YOUR-API-KEY>".to_string()),
                                                ("DD-APPLICATION-KEY".to_string(), "<YOUR-APP-KEY>".to_string()),
                                            ],
                                        ),
                                    )
                                    .mcp_protocol_version(SyntheticsMCPProtocolVersion::VERSION_2025_06_18)
                                    .tool_args(
                                        BTreeMap::from(
                                            [
                                                ("limit".to_string(), Value::from(5)),
                                                ("query".to_string(), Value::from("datadog synthetics")),
                                            ],
                                        ),
                                    )
                                    .tool_name("search".to_string())
                                    .url("https://example.org/mcp".to_string()),
                                SyntheticsAPITestStepSubtype::MCP,
                            )
                                .allow_failure(false)
                                .is_critical(true)
                                .retry(SyntheticsTestOptionsRetry::new().count(0).interval(300.0 as f64)),
                        ),
                    )
                ],
            ),
            vec!["aws:us-east-2".to_string()],
            "BDD test payload: synthetics_api_test_mcp_payload.json".to_string(),
            "Example-Synthetic".to_string(),
            SyntheticsTestOptions::new()
                .min_failure_duration(10)
                .min_location_failed(1)
                .monitor_name("Example-Synthetic".to_string())
                .monitor_priority(5)
                .retry(SyntheticsTestOptionsRetry::new().count(3).interval(1000.0 as f64))
                .tick_every(900),
            SyntheticsAPITestType::API,
        )
            .subtype(SyntheticsTestDetailsSubType::MULTI)
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
