// Create bandit returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsBanditFlagConfigurationInputV2;
use datadog_api_client::datadogV2::model::ExperimentsBanditV2DTODataType;
use datadog_api_client::datadogV2::model::ExperimentsBanditVariantInputV2;
use datadog_api_client::datadogV2::model::ExperimentsCreateBanditV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateBanditV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateBanditV2RequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "feature_flag" in the system
    let feature_flag_data_attributes_variants_0_id = uuid::Uuid::parse_str(
        &std::env::var("FEATURE_FLAG_DATA_ATTRIBUTES_VARIANTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let feature_flag_data_attributes_variants_1_id = uuid::Uuid::parse_str(
        &std::env::var("FEATURE_FLAG_DATA_ATTRIBUTES_VARIANTS_1_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let feature_flag_data_id =
        uuid::Uuid::parse_str(&std::env::var("FEATURE_FLAG_DATA_ID").unwrap())
            .expect("Invalid UUID");

    // there is a valid "environment" in the system
    let environment_data_id = uuid::Uuid::parse_str(&std::env::var("ENVIRONMENT_DATA_ID").unwrap())
        .expect("Invalid UUID");

    // there is a valid "experiment_subject_type" in the system
    let experiment_subject_type_data_id =
        uuid::Uuid::parse_str(&std::env::var("EXPERIMENT_SUBJECT_TYPE_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let body = ExperimentsCreateBanditV2Request::new(ExperimentsCreateBanditV2RequestData::new(
        ExperimentsCreateBanditV2RequestDataAttributes::new("bandit-14bb9543f523edde".to_string())
            .datadog_flag_configuration(Some(
                ExperimentsBanditFlagConfigurationInputV2::new()
                    .environment_id(environment_data_id.clone())
                    .feature_flag_id(feature_flag_data_id.clone())
                    .targeting_rules(vec![]),
            ))
            .subject_type_id(experiment_subject_type_data_id.clone())
            .tags(vec!["bdd".to_string()])
            .variants(vec![
                ExperimentsBanditVariantInputV2::new(
                    feature_flag_data_attributes_variants_0_id.clone(),
                )
                .is_fallback(true),
                ExperimentsBanditVariantInputV2::new(
                    feature_flag_data_attributes_variants_1_id.clone(),
                ),
            ]),
        ExperimentsBanditV2DTODataType::BANDITS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_bandit", true);
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_bandit(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
