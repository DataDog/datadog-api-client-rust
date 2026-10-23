// Create experiment protocol returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentProtocolRequest;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentProtocolRequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentProtocolRequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsPublicProtocolResponseDataType;

#[tokio::main]
async fn main() {
    let body = ExperimentsCreateExperimentProtocolRequest::new(
        ExperimentsCreateExperimentProtocolRequestData::new(
            ExperimentsCreateExperimentProtocolRequestDataAttributes::new(
                "ex-protocol-14bb9543f523edde".to_string(),
            )
            .description(Some("Protocol created by the API test.".to_string())),
            ExperimentsPublicProtocolResponseDataType::PROTOCOLS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_experiment_protocol(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
