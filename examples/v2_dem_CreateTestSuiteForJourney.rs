// Create a test suite for a DEM journey returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dem::DEMAPI;
use datadog_api_client::datadogV2::model::DemCreateJourneyTestSuiteAttributes;
use datadog_api_client::datadogV2::model::DemCreateJourneyTestSuiteData;
use datadog_api_client::datadogV2::model::DemCreateJourneyTestSuiteRequest;
use datadog_api_client::datadogV2::model::DemCreateJourneyTestSuiteRequestType;

#[tokio::main]
async fn main() {
    let body = DemCreateJourneyTestSuiteRequest::new(
        DemCreateJourneyTestSuiteData::new(
            DemCreateJourneyTestSuiteRequestType::CREATE_TEST_SUITE_FOR_JOURNEY_REQUEST,
        )
        .attributes(
            DemCreateJourneyTestSuiteAttributes::new()
                .include_tests_from_journey_coverage(Some(true))
                .test_suite_name(Some("My Custom Suite".to_string())),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = DEMAPI::with_config(configuration);
    let resp = api
        .create_test_suite_for_journey("public_journey_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
