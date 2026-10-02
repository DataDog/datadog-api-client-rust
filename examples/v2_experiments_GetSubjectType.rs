// Get subject type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::GetSubjectTypeOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_subject_type" in the system
    let experiment_subject_type_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_SUBJECT_TYPE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .get_subject_type(
            experiment_subject_type_data_id.clone(),
            GetSubjectTypeOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
