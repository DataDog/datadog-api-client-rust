// Patch subject type returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsPatchSubjectTypeV2Request;
use datadog_api_client::datadogV2::model::ExperimentsPatchSubjectTypeV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsPatchSubjectTypeV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsSubjectTypeV2DTODataType;

#[tokio::main]
async fn main() {
    // there is a valid "experiment_subject_type" in the system
    let experiment_subject_type_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_SUBJECT_TYPE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body =
        ExperimentsPatchSubjectTypeV2Request::new(ExperimentsPatchSubjectTypeV2RequestData::new(
            ExperimentsPatchSubjectTypeV2RequestDataAttributes::new()
                .name("ex-14bb9543f523edde updated".to_string()),
            ExperimentsSubjectTypeV2DTODataType::SUBJECT_TYPES,
        ));
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .patch_subject_type(experiment_subject_type_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
