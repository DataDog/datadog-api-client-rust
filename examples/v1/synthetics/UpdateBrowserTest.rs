// Edit a browser test returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        SyntheticsBrowserTest::new(
            SyntheticsBrowserTestConfig::new(
                vec![],
                SyntheticsTestRequest::new()
                    .basic_auth(
                        SyntheticsBasicAuth::SyntheticsBasicAuthWeb(
                            Box::new(
                                SyntheticsBasicAuthWeb::new(
                                    "PaSSw0RD!".to_string(),
                                    "my_username".to_string(),
                                ).type_(SyntheticsBasicAuthWebType::WEB),
                            ),
                        ),
                    )
                    .body_type(SyntheticsTestRequestBodyType::TEXT_PLAIN)
                    .call_type(SyntheticsTestCallType::UNARY)
                    .certificate(
                        SyntheticsTestRequestCertificate::new()
                            .cert(SyntheticsTestRequestCertificateItem::new())
                            .key(SyntheticsTestRequestCertificateItem::new()),
                    )
                    .certificate_domains(vec![])
                    .proxy(SyntheticsTestRequestProxy::new("https://example.com".to_string()))
                    .service("Greeter".to_string())
                    .url("https://example.com".to_string()),
            )
                .config_variables(
                    vec![
                        SyntheticsConfigVariable::new(
                            "VARIABLE_NAME".to_string(),
                            SyntheticsConfigVariableType::TEXT,
                        ).secure(false)
                    ],
                )
                .variables(
                    vec![
                        SyntheticsBrowserVariable::new(
                            "VARIABLE_NAME".to_string(),
                            SyntheticsBrowserVariableType::TEXT,
                        )
                    ],
                ),
            vec!["aws:eu-west-3".to_string()],
            "".to_string(),
            "Example test name".to_string(),
            SyntheticsTestOptions::new()
                .ci(SyntheticsTestCiOptions::new().execution_rule(SyntheticsTestExecutionRule::BLOCKING))
                .device_ids(vec![SyntheticsDeviceID::LAPTOP_LARGE])
                .http_version(SyntheticsTestOptionsHTTPVersion::HTTP1)
                .monitor_options(SyntheticsTestOptionsMonitorOptions::new())
                .restricted_roles(vec!["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string()])
                .retry(SyntheticsTestOptionsRetry::new())
                .rum_settings(
                    SyntheticsBrowserTestRumSettings::new(true)
                        .application_id("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string())
                        .client_token_id(12345),
                )
                .scheduling(
                    SyntheticsTestOptionsScheduling::new()
                        .timeframes(
                            vec![
                                SyntheticsTestOptionsSchedulingTimeframe::new()
                                    .day(1)
                                    .from("07:00".to_string())
                                    .to("16:00".to_string()),
                                SyntheticsTestOptionsSchedulingTimeframe::new()
                                    .day(3)
                                    .from("07:00".to_string())
                                    .to("16:00".to_string())
                            ],
                        )
                        .timezone("America/New_York".to_string()),
                ),
            SyntheticsBrowserTestType::BROWSER,
        )
            .status(SyntheticsTestPauseStatus::LIVE)
            .steps(vec![SyntheticsStep::new().type_(SyntheticsStepType::ASSERT_ELEMENT_CONTENT)])
            .tags(vec!["env:prod".to_string()]);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.update_browser_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
