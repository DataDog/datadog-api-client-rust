// Create an API test with multi subtype returns "OK - Returns the created test
// details." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsAPIStep;
use datadog_api_client::datadogV1::model::SyntheticsAPITest;
use datadog_api_client::datadogV1::model::SyntheticsAPITestConfig;
use datadog_api_client::datadogV1::model::SyntheticsAPITestStep;
use datadog_api_client::datadogV1::model::SyntheticsAPITestStepSubtype;
use datadog_api_client::datadogV1::model::SyntheticsAPITestType;
use datadog_api_client::datadogV1::model::SyntheticsAPIWaitStep;
use datadog_api_client::datadogV1::model::SyntheticsAPIWaitStepSubtype;
use datadog_api_client::datadogV1::model::SyntheticsAssertion;
use datadog_api_client::datadogV1::model::SyntheticsAssertionOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTargetValue;
use datadog_api_client::datadogV1::model::SyntheticsAssertionType;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuth;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuthWeb;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuthWebType;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariable;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariableType;
use datadog_api_client::datadogV1::model::SyntheticsGlobalVariableParserType;
use datadog_api_client::datadogV1::model::SyntheticsLocalVariableParsingOptionsType;
use datadog_api_client::datadogV1::model::SyntheticsParsingOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestCallType;
use datadog_api_client::datadogV1::model::SyntheticsTestDetailsSubType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsHTTPVersion;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestDNSServerPort;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestPort;
use datadog_api_client::datadogV1::model::SyntheticsVariableParser;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsAPITest::new(
            SyntheticsAPITestConfig::new()
                .config_variables(
                    vec![
                        SyntheticsConfigVariable::new("PROPERTY".to_string(), SyntheticsConfigVariableType::TEXT)
                            .example("content-type".to_string())
                            .pattern("content-type".to_string())
                    ],
                )
                .steps(
                    vec![
                        SyntheticsAPIStep::SyntheticsAPITestStep(
                            Box::new(
                                SyntheticsAPITestStep::new(
                                    vec![
                                        SyntheticsAssertion::SyntheticsAssertionTarget(
                                            Box::new(
                                                SyntheticsAssertionTarget::new(
                                                    SyntheticsAssertionOperator::IS,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        200.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::STATUS_CODE,
                                                ),
                                            ),
                                        )
                                    ],
                                    "request is sent".to_string(),
                                    SyntheticsTestRequest::new()
                                        .http_version(SyntheticsTestOptionsHTTPVersion::HTTP2)
                                        .method("GET".to_string())
                                        .timeout(10.0 as f64)
                                        .url("https://datadoghq.com".to_string()),
                                    SyntheticsAPITestStepSubtype::HTTP,
                                )
                                    .allow_failure(true)
                                    .exit_if_succeed(true)
                                    .extracted_values(
                                        vec![
                                            SyntheticsParsingOptions::new()
                                                .field("server".to_string())
                                                .name("EXTRACTED_VALUE".to_string())
                                                .parser(
                                                    SyntheticsVariableParser::new(
                                                        SyntheticsGlobalVariableParserType::RAW,
                                                    ),
                                                )
                                                .secure(true)
                                                .type_(SyntheticsLocalVariableParsingOptionsType::HTTP_HEADER)
                                        ],
                                    )
                                    .extracted_values_from_script(
                                        "dd.variable.set('STATUS_CODE', dd.response.statusCode);".to_string(),
                                    )
                                    .is_critical(true)
                                    .retry(SyntheticsTestOptionsRetry::new().count(5).interval(1000.0 as f64)),
                            ),
                        ),
                        SyntheticsAPIStep::SyntheticsAPIWaitStep(
                            Box::new(
                                SyntheticsAPIWaitStep::new("Wait".to_string(), SyntheticsAPIWaitStepSubtype::WAIT, 1),
                            ),
                        ),
                        SyntheticsAPIStep::SyntheticsAPITestStep(
                            Box::new(
                                SyntheticsAPITestStep::new(
                                    vec![
                                        SyntheticsAssertion::SyntheticsAssertionTarget(
                                            Box::new(
                                                SyntheticsAssertionTarget::new(
                                                    SyntheticsAssertionOperator::LESS_THAN,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        1000.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::RESPONSE_TIME,
                                                ),
                                            ),
                                        )
                                    ],
                                    "GRPC CALL".to_string(),
                                    SyntheticsTestRequest::new()
                                        .call_type(SyntheticsTestCallType::UNARY)
                                        .compressed_json_descriptor(
                                            "eJy1lU1z2yAQhv+Lzj74I3ETH506bQ7OZOSm1w4Wa4epBARQppqM/3v5koCJJdvtxCdW77vPssCO3zMKUgHOFu/ZXvBiS6hZho/f8qe7pftYgXphWJrlA8XwxywEvNba+6PhkC2yVcVVswYp0R6ykRYlZ1SCV21SDrxsssPIeS9FJKqGfK2rqnmmSBwhWa2XlKgtaQPiDcRGCUDVfwGD2sKUqKEtc1cSoOrsMlaMOec1sySYCCgUYRSVLv2zSva2u+FQkB0pVkIw8bFuIudOOn3pOaKYVT3Iy97Pd0AYhOx5QcMsnxvRHlnuLf8ETDd3CNtrv2nejkDpRnANCmGkkFn/hsYzpBKE7jVbufgnKnV9HRM9zRPDDKPttYT61n0TdWkAAjggk9AhuxIeaXd69CYTcsGw7cBTakLVbNpRzGEgyWjkSOpMbZXkhGL6oX30R49qt3GoHrap7i0XdD41WQ+2icCNm5p1hmFqnHNlcla0riKmDZ183crDxChjbnurtxHPRE784sVhWvDfGP+SsTKibU3o5NtWHuZFGZOxP6P5VXqIOvaOSec4eYohyd7NslHuJbd1bewds85xYrNxkr2d+5IhFWF3NvaO684xjE2S5ulY+tu64Pna0fCPJgzw6vF5/WucLcYjt5xoq19O3UDptOg/OamJQRaCcPPnMTQ2QDFn+uhPvUfnCrMc99upyQY4Ui9Dlc/YoG3R/v4Cs9YE+g==".to_string(),
                                        )
                                        .host("grpcbin.test.k6.io".to_string())
                                        .message("{}".to_string())
                                        .metadata(BTreeMap::from([]))
                                        .method("Index".to_string())
                                        .port(SyntheticsTestRequestPort::SyntheticsTestRequestNumericalPort(9000))
                                        .service("grpcbin.GRPCBin".to_string()),
                                    SyntheticsAPITestStepSubtype::GRPC,
                                )
                                    .allow_failure(false)
                                    .extracted_values(vec![])
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
                                                    SyntheticsAssertionOperator::IS_IN_MORE_DAYS_THAN,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        10.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::CERTIFICATE,
                                                ),
                                            ),
                                        )
                                    ],
                                    "SSL step".to_string(),
                                    SyntheticsTestRequest::new()
                                        .check_certificate_revocation(true)
                                        .disable_aia_intermediate_fetching(true)
                                        .host("example.org".to_string())
                                        .port(SyntheticsTestRequestPort::SyntheticsTestRequestNumericalPort(443)),
                                    SyntheticsAPITestStepSubtype::SSL,
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
                                                    SyntheticsAssertionOperator::LESS_THAN,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        1000.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::RESPONSE_TIME,
                                                ),
                                            ),
                                        )
                                    ],
                                    "DNS step".to_string(),
                                    SyntheticsTestRequest::new()
                                        .dns_server("8.8.8.8".to_string())
                                        .dns_server_port(
                                            SyntheticsTestRequestDNSServerPort
                                            ::SyntheticsTestRequestVariableDNSServerPort(
                                                "53".to_string(),
                                            ),
                                        )
                                        .host("troisdizaines.com".to_string()),
                                    SyntheticsAPITestStepSubtype::DNS,
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
                                                    SyntheticsAssertionOperator::LESS_THAN,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        1000.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::RESPONSE_TIME,
                                                ),
                                            ),
                                        )
                                    ],
                                    "TCP step".to_string(),
                                    SyntheticsTestRequest::new()
                                        .host("34.95.79.70".to_string())
                                        .port(SyntheticsTestRequestPort::SyntheticsTestRequestNumericalPort(80))
                                        .should_track_hops(true)
                                        .timeout(32.0 as f64),
                                    SyntheticsAPITestStepSubtype::TCP,
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
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        0.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::PACKET_LOSS_PERCENTAGE,
                                                ),
                                            ),
                                        )
                                    ],
                                    "ICMP step".to_string(),
                                    SyntheticsTestRequest::new()
                                        .host("34.95.79.70".to_string())
                                        .number_of_packets(4)
                                        .should_track_hops(true)
                                        .timeout(38.0 as f64),
                                    SyntheticsAPITestStepSubtype::ICMP,
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
                                                    SyntheticsAssertionOperator::LESS_THAN,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        1000.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::RESPONSE_TIME,
                                                ),
                                            ),
                                        )
                                    ],
                                    "Websocket step".to_string(),
                                    SyntheticsTestRequest::new()
                                        .basic_auth(
                                            SyntheticsBasicAuth::SyntheticsBasicAuthWeb(
                                                Box::new(
                                                    SyntheticsBasicAuthWeb::new()
                                                        .password("password".to_string())
                                                        .type_(SyntheticsBasicAuthWebType::WEB)
                                                        .username("user".to_string()),
                                                ),
                                            ),
                                        )
                                        .headers(BTreeMap::from([("f".to_string(), "g".to_string())]))
                                        .is_message_base64_encoded(true)
                                        .message("My message".to_string())
                                        .url("ws://34.95.79.70/web-socket".to_string()),
                                    SyntheticsAPITestStepSubtype::WEBSOCKET,
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
                                                    SyntheticsAssertionOperator::LESS_THAN,
                                                    SyntheticsAssertionTargetValue
                                                    ::SyntheticsAssertionTargetValueNumber(
                                                        1000.0 as f64,
                                                    ),
                                                    SyntheticsAssertionType::RESPONSE_TIME,
                                                ),
                                            ),
                                        )
                                    ],
                                    "UDP step".to_string(),
                                    SyntheticsTestRequest::new()
                                        .host("8.8.8.8".to_string())
                                        .message("A image.google.com".to_string())
                                        .port(SyntheticsTestRequestPort::SyntheticsTestRequestNumericalPort(53)),
                                    SyntheticsAPITestStepSubtype::UDP,
                                )
                                    .allow_failure(false)
                                    .is_critical(true)
                                    .retry(SyntheticsTestOptionsRetry::new().count(0).interval(300.0 as f64)),
                            ),
                        )
                    ],
                ),
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
                .retry(SyntheticsTestOptionsRetry::new().count(3).interval(1000.0 as f64))
                .tick_every(60),
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
