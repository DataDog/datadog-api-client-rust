// Create an API test with a file payload returns "OK - Returns the created test
// details." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsAPITest;
use datadog_api_client::datadogV1::model::SyntheticsAPITestConfig;
use datadog_api_client::datadogV1::model::SyntheticsAPITestType;
use datadog_api_client::datadogV1::model::SyntheticsAssertion;
use datadog_api_client::datadogV1::model::SyntheticsAssertionJSONPathOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionJSONPathTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionJSONPathTargetTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTargetValue;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTimingsScope;
use datadog_api_client::datadogV1::model::SyntheticsAssertionType;
use datadog_api_client::datadogV1::model::SyntheticsAssertionXPathOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionXPathTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionXPathTargetTarget;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuth;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuthOauthClient;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuthOauthClientType;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuthOauthTokenApiAuthentication;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariable;
use datadog_api_client::datadogV1::model::SyntheticsConfigVariableType;
use datadog_api_client::datadogV1::model::SyntheticsTestDetailsSubType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsHTTPVersion;
use datadog_api_client::datadogV1::model::SyntheticsTestOptionsRetry;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestBodyFile;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestBodyType;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestCertificate;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestCertificateItem;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestProxy;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsAPITest::new(
            SyntheticsAPITestConfig::new()
                .assertions(
                    vec![
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::IS,
                                    SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueString(
                                        "text/html".to_string(),
                                    ),
                                    SyntheticsAssertionType::HEADER,
                                ).property("{{ PROPERTY }}".to_string()),
                            ),
                        ),
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::LESS_THAN,
                                    SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                                        2000.0 as f64,
                                    ),
                                    SyntheticsAssertionType::RESPONSE_TIME,
                                ).timings_scope(SyntheticsAssertionTimingsScope::WITHOUT_DNS),
                            ),
                        ),
                        SyntheticsAssertion::SyntheticsAssertionJSONPathTarget(
                            Box::new(
                                SyntheticsAssertionJSONPathTarget::new(
                                    SyntheticsAssertionJSONPathOperator::VALIDATES_JSON_PATH,
                                    SyntheticsAssertionType::BODY,
                                ).target(
                                    SyntheticsAssertionJSONPathTargetTarget::new()
                                        .json_path("topKey".to_string())
                                        .operator("isNot".to_string())
                                        .target_value(
                                            SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueString(
                                                "0".to_string(),
                                            ),
                                        ),
                                ),
                            ),
                        ),
                        SyntheticsAssertion::SyntheticsAssertionXPathTarget(
                            Box::new(
                                SyntheticsAssertionXPathTarget::new(
                                    SyntheticsAssertionXPathOperator::VALIDATES_X_PATH,
                                    SyntheticsAssertionType::BODY,
                                ).target(
                                    SyntheticsAssertionXPathTargetTarget::new()
                                        .operator("contains".to_string())
                                        .target_value(
                                            SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueString(
                                                "0".to_string(),
                                            ),
                                        )
                                        .x_path("target-xpath".to_string()),
                                ),
                            ),
                        )
                    ],
                )
                .config_variables(
                    vec![
                        SyntheticsConfigVariable::new("PROPERTY".to_string(), SyntheticsConfigVariableType::TEXT)
                            .example("content-type".to_string())
                            .pattern("content-type".to_string())
                    ],
                )
                .request(
                    SyntheticsTestRequest::new()
                        .basic_auth(
                            SyntheticsBasicAuth::SyntheticsBasicAuthOauthClient(
                                Box::new(
                                    SyntheticsBasicAuthOauthClient::new(
                                        "https://datadog-token.com".to_string(),
                                        "client-id".to_string(),
                                        "client-secret".to_string(),
                                        SyntheticsBasicAuthOauthTokenApiAuthentication::HEADER,
                                        SyntheticsBasicAuthOauthClientType::OAUTH_CLIENT,
                                    )
                                        .audience("audience".to_string())
                                        .resource("resource".to_string())
                                        .scope("yoyo".to_string()),
                                ),
                            ),
                        )
                        .body_type(SyntheticsTestRequestBodyType::APPLICATION_OCTET_STREAM)
                        .certificate(
                            SyntheticsTestRequestCertificate::new()
                                .cert(
                                    SyntheticsTestRequestCertificateItem::new()
                                        .content("cert-content".to_string())
                                        .filename("cert-filename".to_string())
                                        .updated_at("2020-10-16T09:23:24.857Z".to_string()),
                                )
                                .key(
                                    SyntheticsTestRequestCertificateItem::new()
                                        .content("key-content".to_string())
                                        .filename("key-filename".to_string())
                                        .updated_at("2020-10-16T09:23:24.857Z".to_string()),
                                ),
                        )
                        .files(
                            vec![
                                SyntheticsTestRequestBodyFile::new()
                                    .content("file content".to_string())
                                    .name("file name".to_string())
                                    .original_file_name("image.png".to_string())
                                    .type_("file type".to_string())
                            ],
                        )
                        .headers(BTreeMap::from([("unique".to_string(), "examplesynthetic".to_string())]))
                        .method("GET".to_string())
                        .persist_cookies(true)
                        .proxy(
                            SyntheticsTestRequestProxy::new(
                                "https://datadoghq.com".to_string(),
                            ).headers(BTreeMap::from([])),
                        )
                        .timeout(10.0 as f64)
                        .url("https://datadoghq.com".to_string()),
                ),
            vec!["aws:us-east-2".to_string()],
            "BDD test payload: synthetics_api_http_test_payload.json".to_string(),
            "Example-Synthetic".to_string(),
            SyntheticsTestOptions::new()
                .accept_self_signed(false)
                .allow_insecure(true)
                .follow_redirects(true)
                .http_version(SyntheticsTestOptionsHTTPVersion::HTTP2)
                .min_failure_duration(10)
                .min_location_failed(1)
                .monitor_name("Example-Synthetic".to_string())
                .monitor_priority(5)
                .retry(SyntheticsTestOptionsRetry::new().count(3).interval(10.0 as f64))
                .tick_every(60),
            SyntheticsAPITestType::API,
        )
            .subtype(SyntheticsTestDetailsSubType::HTTP)
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
