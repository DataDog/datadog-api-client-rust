// List experiment protocols returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::ListExperimentProtocolsOptionalParams;
use datadog_api_client::datadogV2::model::ExperimentsPublicProtocolResponseDataAttributesStatus;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_protocol" in the system
    let experiment_protocol_data_attributes_name =
        std::env::var("EXPERIMENT_PROTOCOL_DATA_ATTRIBUTES_NAME").unwrap();
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .list_experiment_protocols(
            ListExperimentProtocolsOptionalParams::default()
                .filter_status(vec![
                    ExperimentsPublicProtocolResponseDataAttributesStatus::DRAFT,
                ])
                .filter_query(experiment_protocol_data_attributes_name.clone()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
