// Get a pipeline preserves end-to-end acknowledgements returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "pipeline with end-to-end acknowledgements" in the system
    let pipeline_with_end_to_end_acknowledgements_data_id =
        std::env::var("PIPELINE_WITH_END_TO_END_ACKNOWLEDGEMENTS_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ObservabilityPipelinesAPI::with_config(configuration);
    let resp = api
        .get_pipeline(pipeline_with_end_to_end_acknowledgements_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
