// Create experiment metric group from collection returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "experiment" in the system
    let experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_DATA_ID").unwrap()).expect("Invalid UUID");

    // there is a valid "experiment_metric_collection_with_metric" in the system
    let experiment_metric_collection_with_metric_data_id = uuid::Uuid::parse_str(
        &std::env::var("EXPERIMENT_METRIC_COLLECTION_WITH_METRIC_DATA_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .create_experiment_metric_group_from_collection(
            experiment_data_id.clone(),
            experiment_metric_collection_with_metric_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
