// Unarchive exposure SQL model returns "The exposure SQL model was unarchived.
// Unarchiving a model that is not archived succeeds and changes nothing." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .unarchive_exposure_sql_model(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
