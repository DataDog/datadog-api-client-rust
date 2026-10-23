// Get experiment protocol returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_protocol" in the system
    let experiment_protocol_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_PROTOCOL_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .get_experiment_protocol(experiment_protocol_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
