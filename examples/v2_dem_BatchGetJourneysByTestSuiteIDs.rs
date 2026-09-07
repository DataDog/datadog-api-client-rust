// Batch get DEM journeys by test suite IDs returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dem::DEMAPI;
use datadog_api_client::datadogV2::model::DemBatchGetJourneysAttributes;
use datadog_api_client::datadogV2::model::DemBatchGetJourneysData;
use datadog_api_client::datadogV2::model::DemBatchGetJourneysRequest;
use datadog_api_client::datadogV2::model::DemBatchGetJourneysRequestType;

#[tokio::main]
async fn main() {
    let body = DemBatchGetJourneysRequest::new(DemBatchGetJourneysData::new(
        DemBatchGetJourneysAttributes::new(vec![
            "suite-abc123".to_string(),
            "suite-def456".to_string(),
        ]),
        DemBatchGetJourneysRequestType::BATCH_GET_JOURNEYS_BY_TEST_SUITE_IDS_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = DEMAPI::with_config(configuration);
    let resp = api.batch_get_journeys_by_test_suite_i_ds(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
