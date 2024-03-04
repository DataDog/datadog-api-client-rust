// Create an API HTTP with oauth-rop test returns "OK - Returns the created test details." response
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
        SyntheticsAPITest::new(
            SyntheticsAPITestConfig::new()
                .assertions(
                    vec![
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::IS,
                                    "text/html".to_string(),
                                    SyntheticsAssertionType::HEADER,
                                ).property("{{ PROPERTY }}".to_string()),
                            ),
                        ),
                        SyntheticsAssertion::SyntheticsAssertionTarget(
                            Box::new(
                                SyntheticsAssertionTarget::new(
                                    SyntheticsAssertionOperator::LESS_THAN,
                                    2000,
                                    SyntheticsAssertionType::RESPONSE_TIME,
                                ),
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
                                        .target_value("0".to_string()),
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
                                        .target_value("0".to_string())
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
                            SyntheticsBasicAuth::SyntheticsBasicAuthOauthROP(
                                Box::new(
                                    SyntheticsBasicAuthOauthROP::new(
                                        "https://datadog-token.com".to_string(),
                                        "oauth-password".to_string(),
                                        SyntheticsBasicAuthOauthTokenApiAuthentication::BODY,
                                        "oauth-usermame".to_string(),
                                    )
                                        .audience("audience".to_string())
                                        .client_id("client-id".to_string())
                                        .client_secret("client-secret".to_string())
                                        .resource("resource".to_string())
                                        .scope("yoyo".to_string())
                                        .type_(SyntheticsBasicAuthOauthROPType::OAUTH_ROP),
                                ),
                            ),
                        )
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
                        .headers(
                            std::collections::BTreeMap::from(
                                [("unique".to_string(), "examplesynthetic".to_string())],
                            ),
                        )
                        .method("GET".to_string())
                        .proxy(
                            SyntheticsTestRequestProxy::new(
                                "https://datadoghq.com".to_string(),
                            ).headers(std::collections::BTreeMap::from([])),
                        )
                        .timeout(10)
                        .url("https://datadoghq.com".to_string()),
                ),
            vec!["aws:us-east-2".to_string()],
            "BDD test payload: synthetics_api_http_test_payload.json".to_string(),
            "Example-Synthetic".to_string(),
            SyntheticsTestOptions::new()
                .accept_self_signed(false)
                .allow_insecure(true)
                .follow_redirects(true)
                .min_failure_duration(10)
                .min_location_failed(1)
                .monitor_name("Example-Synthetic".to_string())
                .monitor_priority(5)
                .retry(SyntheticsTestOptionsRetry::new().count(3).interval(10))
                .tick_every(60),
            SyntheticsAPITestType::API,
        )
            .subtype(SyntheticsTestDetailsSubType::HTTP)
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
