// Edit a Mobile test returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::SyntheticsMobileTest;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestConfig;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestOptions;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestType;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestsMobileApplication;
use datadog_api_client::datadogV1::model::SyntheticsMobileTestsMobileApplicationReferenceType;
use datadog_api_client::datadogV1::model::SyntheticsTestPauseStatus;

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_mobile_test" in the system
    let synthetics_mobile_test_public_id =
        std::env::var("SYNTHETICS_MOBILE_TEST_PUBLIC_ID").unwrap();
    let body = SyntheticsMobileTest::new(
        SyntheticsMobileTestConfig::new().variables(vec![]),
        "Example-Synthetic-updated".to_string(),
        SyntheticsMobileTestOptions::new()
            .device_ids(vec!["synthetics:mobile:device:iphone_15_ios_17".to_string()])
            .mobile_application(
                SyntheticsMobileTestsMobileApplication::new()
                    .application_id("ab0e0aed-536d-411a-9a99-5428c27d8f8e".to_string())
                    .reference_id("6115922a-5f5d-455e-bc7e-7955a57f3815".to_string())
                    .reference_type(SyntheticsMobileTestsMobileApplicationReferenceType::VERSION),
            )
            .tick_every(3600),
        SyntheticsMobileTestType::MOBILE,
    )
    .message("".to_string())
    .status(SyntheticsTestPauseStatus::PAUSED)
    .steps(vec![]);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .update_mobile_test(synthetics_mobile_test_public_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
