// Synthetics: Create a test suite returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SuiteCreateEdit;
use datadog_api_client::datadogV2::model::SuiteCreateEditRequest;
use datadog_api_client::datadogV2::model::SyntheticsSuite;
use datadog_api_client::datadogV2::model::SyntheticsSuiteOptions;
use datadog_api_client::datadogV2::model::SyntheticsSuiteType;
use datadog_api_client::datadogV2::model::SyntheticsSuiteTypes;

#[tokio::main]
async fn main() {
    let body = SuiteCreateEditRequest::new(SuiteCreateEdit::new(
        SyntheticsSuite::new(
            "Example suite name".to_string(),
            SyntheticsSuiteOptions::new(),
            vec![],
            SyntheticsSuiteType::SUITE,
        )
        .message("Notification message".to_string())
        .tags(vec!["env:production".to_string()]),
        SyntheticsSuiteTypes::SUITES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_synthetics_suite(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
