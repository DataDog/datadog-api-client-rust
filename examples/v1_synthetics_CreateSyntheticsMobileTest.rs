// Create a mobile test returns "OK - Returns the created test details." response
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
    let body = SyntheticsMobileTest::new(
        SyntheticsMobileTestConfig::new().variables(vec![]),
        "".to_string(),
        "Example-Synthetic".to_string(),
        SyntheticsMobileTestOptions::new(
            vec!["synthetics:mobile:device:iphone_15_ios_17".to_string()],
            SyntheticsMobileTestsMobileApplication::new(
                "ab0e0aed-536d-411a-9a99-5428c27d8f8e".to_string(),
                "6115922a-5f5d-455e-bc7e-7955a57f3815".to_string(),
                SyntheticsMobileTestsMobileApplicationReferenceType::VERSION,
            ),
            3600,
        ),
        SyntheticsMobileTestType::MOBILE,
    )
    .status(SyntheticsTestPauseStatus::PAUSED)
    .steps(vec![]);
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_mobile_test(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
