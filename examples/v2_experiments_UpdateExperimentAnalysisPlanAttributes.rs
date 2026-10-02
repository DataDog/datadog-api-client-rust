// Update experiment analysis plan attributes returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsAnalysisPlanWriteV2Request;
use datadog_api_client::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsAnalysisPlanWriteV2RequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment" in the system
    let experiment_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = ExperimentsAnalysisPlanWriteV2Request::new(
        ExperimentsAnalysisPlanWriteV2RequestData::new(
            ExperimentsAnalysisPlanWriteV2RequestDataType::ANALYSIS_PLANS,
        )
        .attributes(
            ExperimentsAnalysisPlanWriteV2RequestDataAttributes::new().confidence_level(0.9 as f64),
        )
        .id(experiment_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .update_experiment_analysis_plan_attributes(experiment_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
