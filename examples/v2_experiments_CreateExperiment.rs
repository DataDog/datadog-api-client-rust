// Create experiment returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataType;

#[tokio::main]
async fn main() {
    let body =
        ExperimentsCreateExperimentV2Request::new(ExperimentsCreateExperimentV2RequestData::new(
            ExperimentsCreateExperimentV2RequestDataAttributes::new(
                "ex-14bb9543f523edde".to_string(),
            ),
            ExperimentsPatchExperimentV2ResponseDataType::EXPERIMENTS,
        ));
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_experiment(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
