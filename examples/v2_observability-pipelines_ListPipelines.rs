// List pipelines returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_observability_pipelines::ListPipelinesOptionalParams;
use datadog_api_client::datadogV2::api_observability_pipelines::ObservabilityPipelinesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListPipelines", true);
    let api = ObservabilityPipelinesAPI::with_config(configuration);
    let resp = api
        .list_pipelines(ListPipelinesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
