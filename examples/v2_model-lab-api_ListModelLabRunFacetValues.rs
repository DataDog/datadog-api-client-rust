// List Model Lab run facet values returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_model_lab_api::ModelLabAPIAPI;
use datadog_api_client::datadogV2::model::ModelLabFacetType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListModelLabRunFacetValues", true);
    let api = ModelLabAPIAPI::with_config(configuration);
    let resp = api
        .list_model_lab_run_facet_values(2387, ModelLabFacetType::TAG, "model".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
