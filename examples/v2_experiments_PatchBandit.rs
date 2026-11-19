// Update bandit returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsBanditV2DTODataType;
use datadog_api_client::datadogV2::model::ExperimentsBanditVariantInputV2;
use datadog_api_client::datadogV2::model::ExperimentsPatchBanditV2Request;
use datadog_api_client::datadogV2::model::ExperimentsPatchBanditV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsPatchBanditV2RequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "bandit" in the system
    let bandit_data_id =
        uuid::Uuid::parse_str(&std::env::var("BANDIT_DATA_ID").unwrap()).expect("Invalid UUID");

    // there is a valid "feature_flag" in the system
    let feature_flag_data_attributes_variants_0_id = uuid::Uuid::parse_str(
        &std::env::var("FEATURE_FLAG_DATA_ATTRIBUTES_VARIANTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let feature_flag_data_attributes_variants_1_id = uuid::Uuid::parse_str(
        &std::env::var("FEATURE_FLAG_DATA_ATTRIBUTES_VARIANTS_1_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let body = ExperimentsPatchBanditV2Request::new(
        ExperimentsPatchBanditV2RequestData::new(
            ExperimentsPatchBanditV2RequestDataAttributes::new()
                .name("bandit-14bb9543f523edde-renamed".to_string())
                .variants(vec![
                    ExperimentsBanditVariantInputV2::new(
                        feature_flag_data_attributes_variants_0_id.clone(),
                    ),
                    ExperimentsBanditVariantInputV2::new(
                        feature_flag_data_attributes_variants_1_id.clone(),
                    )
                    .is_fallback(true),
                ]),
            ExperimentsBanditV2DTODataType::BANDITS,
        )
        .id(bandit_data_id.clone()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_bandit", true);
    configuration.set_unstable_operation_enabled("v2.patch_bandit", true);
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.patch_bandit(bandit_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
