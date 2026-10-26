// Replace experiment protocol returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::api_experiments::ReplaceExperimentProtocolOptionalParams;
use datadog_api_client::datadogV2::model::ExperimentsPublicProtocolResponseDataType;
use datadog_api_client::datadogV2::model::ExperimentsReplaceExperimentProtocolRequest;
use datadog_api_client::datadogV2::model::ExperimentsReplaceExperimentProtocolRequestData;
use datadog_api_client::datadogV2::model::ExperimentsReplaceExperimentProtocolRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "published_protocol" in the system
    let published_protocol_data_attributes_updated_at =
        std::env::var("PUBLISHED_PROTOCOL_DATA_ATTRIBUTES_UPDATED_AT").unwrap();
    let published_protocol_data_id =
        uuid::Uuid::parse_str(&std::env::var("PUBLISHED_PROTOCOL_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "experiment_subject_type" in the system
    let experiment_subject_type_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_SUBJECT_TYPE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = ExperimentsReplaceExperimentProtocolRequest::new(
        ExperimentsReplaceExperimentProtocolRequestData::new(
            ExperimentsReplaceExperimentProtocolRequestDataAttributes::new(
                false,
                false,
                false,
                false,
                vec![],
                experiment_subject_type_data_id.clone(),
            )
            .default_duration_days(Some(14)),
            ExperimentsPublicProtocolResponseDataType::PROTOCOLS,
        )
        .id(published_protocol_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .replace_experiment_protocol(
            published_protocol_data_id.clone(),
            body,
            ReplaceExperimentProtocolOptionalParams::default()
                .expected_updated_at(published_protocol_data_attributes_updated_at.clone()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
