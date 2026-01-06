// Synthetics: edit a test suite returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::SuiteCreateEdit;
use datadog_api_client::datadogV2::model::SuiteCreateEditRequest;
use datadog_api_client::datadogV2::model::SyntheticsSuite;
use datadog_api_client::datadogV2::model::SyntheticsSuiteOptions;
use datadog_api_client::datadogV2::model::SyntheticsSuiteTest;
use datadog_api_client::datadogV2::model::SyntheticsSuiteTestAlertingCriticality;
use datadog_api_client::datadogV2::model::SyntheticsSuiteType;
use datadog_api_client::datadogV2::model::SyntheticsSuiteTypes;

#[tokio::main]
async fn main() {
    let body = SuiteCreateEditRequest::new(SuiteCreateEdit::new(
        SyntheticsSuite::new(
            "Notification message".to_string(),
            "Example suite name".to_string(),
            SyntheticsSuiteOptions::new(),
            vec![SyntheticsSuiteTest::new("".to_string())
                .alerting_criticality(SyntheticsSuiteTestAlertingCriticality::CRITICAL)],
            SyntheticsSuiteType::SUITE,
        )
        .tags(vec!["env:production".to_string()]),
        SyntheticsSuiteTypes::SUITES,
    ));
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .edit_synthetics_suite("public_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
