// Create an API SSL test returns "OK - Returns the created test details." response
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
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;
use datadog_api_client::datadogV1::model::SyntheticsTestRequestPort;

#[tokio::main]
async fn main() {
    let body = SyntheticsAPITest::new(
        SyntheticsAPITestConfig::new()
            .assertions(vec![SyntheticsAssertion::SyntheticsAssertionTarget(
                Box::new(SyntheticsAssertionTarget::new(
                    SyntheticsAssertionOperator::IS_IN_MORE_DAYS_THAN,
                    SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                        10.0 as f64,
                    ),
                    SyntheticsAssertionType::CERTIFICATE,
                )),
            )])
            .request(
                SyntheticsTestRequest::new()
                    .host("datadoghq.com".to_string())
                    .port(
                        SyntheticsTestRequestPort::SyntheticsTestRequestVariablePort(
                            "{{ DATADOG_PORT }}".to_string(),
                        ),
                    ),
            ),
        vec!["aws:us-east-2".to_string()],
        "BDD test payload: synthetics_api_ssl_test_payload.json".to_string(),
        "Example-Synthetic".to_string(),
        SyntheticsTestOptions::new()
            .accept_self_signed(true)
            .check_certificate_revocation(true)
            .tick_every(60),
        SyntheticsAPITestType::API,
    )
    .subtype(SyntheticsTestDetailsSubType::SSL)
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
