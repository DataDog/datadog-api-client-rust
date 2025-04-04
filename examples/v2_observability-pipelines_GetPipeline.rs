// Get a specific pipeline returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "pipeline" in the system
    let pipeline_data_id = std::env::var("PIPELINE_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetPipeline", true);
    let api = ObservabilityPipelinesAPI::with_config(configuration);
    let resp = api.get_pipeline(pipeline_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
