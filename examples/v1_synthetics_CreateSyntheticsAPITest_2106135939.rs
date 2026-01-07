// Create a multistep test with subtest returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsAPIStep;
use datadog_api_client::datadogV1::model::SyntheticsAPISubtestStep;
use datadog_api_client::datadogV1::model::SyntheticsAPISubtestStepSubtype;
use datadog_api_client::datadogV1::model::SyntheticsAPITest;
use datadog_api_client::datadogV1::model::SyntheticsAPITestConfig;
use datadog_api_client::datadogV1::model::SyntheticsAPITestStep;
use datadog_api_client::datadogV1::model::SyntheticsAPITestStepSubtype;
use datadog_api_client::datadogV1::model::SyntheticsAPITestType;
use datadog_api_client::datadogV1::model::SyntheticsAssertion;
use datadog_api_client::datadogV1::model::SyntheticsAssertionOperator;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTarget;
use datadog_api_client::datadogV1::model::SyntheticsAssertionTargetValue;
use datadog_api_client::datadogV1::model::SyntheticsAssertionType;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuth;
use datadog_api_client::datadogV1::model::SyntheticsBasicAuthWeb;
use datadog_api_client::datadogV1::model::SyntheticsTestDetailsSubType;
use datadog_api_client::datadogV1::model::SyntheticsTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsTestRequest;

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_api_test" in the system
    let synthetics_api_test_public_id = std::env::var("SYNTHETICS_API_TEST_PUBLIC_ID").unwrap();
    let body = SyntheticsAPITest::new(
        SyntheticsAPITestConfig::new().steps(vec![
            SyntheticsAPIStep::SyntheticsAPITestStep(Box::new(SyntheticsAPITestStep::new(
                vec![SyntheticsAssertion::SyntheticsAssertionTarget(Box::new(
                    SyntheticsAssertionTarget::new(
                        SyntheticsAssertionOperator::IS,
                        SyntheticsAssertionTargetValue::SyntheticsAssertionTargetValueNumber(
                            200.0 as f64,
                        ),
                        SyntheticsAssertionType::STATUS_CODE,
                    ),
                ))],
                "request is sent".to_string(),
                SyntheticsTestRequest::new()
                    .basic_auth(SyntheticsBasicAuth::SyntheticsBasicAuthWeb(Box::new(
                        SyntheticsBasicAuthWeb::new()
                            .password("password".to_string())
                            .username("username".to_string()),
                    )))
                    .method("GET".to_string())
                    .url("https://httpbin.org/status/200".to_string()),
                SyntheticsAPITestStepSubtype::HTTP,
            ))),
            SyntheticsAPIStep::SyntheticsAPISubtestStep(Box::new(SyntheticsAPISubtestStep::new(
                "subtest step".to_string(),
                synthetics_api_test_public_id.clone(),
                SyntheticsAPISubtestStepSubtype::PLAY_SUB_TEST,
            ))),
        ]),
        vec!["aws:us-east-2".to_string()],
        "BDD test payload: synthetics_api_test_multi_step_with_subtest.json".to_string(),
        "Example-Synthetic".to_string(),
        SyntheticsTestOptions::new().tick_every(60),
        SyntheticsAPITestType::API,
    )
    .subtype(SyntheticsTestDetailsSubType::MULTI);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_api_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
