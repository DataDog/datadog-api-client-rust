// Delete metric collection returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_metric_collection" in the system
    let experiment_metric_collection_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_METRIC_COLLECTION_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .delete_metric_collection(experiment_metric_collection_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
