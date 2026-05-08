// Update a dataset restriction returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dataset_restrictions::DatasetRestrictionsAPI;
use datadog_api_client::datadogV2::model::DatasetRestrictionOwnershipMode;
use datadog_api_client::datadogV2::model::DatasetRestrictionRestrictionMode;
use datadog_api_client::datadogV2::model::DatasetRestrictionUpdateRequest;
use datadog_api_client::datadogV2::model::DatasetRestrictionUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::DatasetRestrictionUpdateRequestData;
use datadog_api_client::datadogV2::model::DatasetRestrictionsType;

#[tokio::main]
async fn main() {
    let body = DatasetRestrictionUpdateRequest::new(DatasetRestrictionUpdateRequestData::new(
        DatasetRestrictionUpdateRequestAttributes::new(
            DatasetRestrictionRestrictionMode::DEFAULT_HIDE,
        )
        .ownership_mode(DatasetRestrictionOwnershipMode::TEAM_TAG_BASED)
        .unrestricted_principals(vec![]),
        DatasetRestrictionsType::DATASET_RESTRICTIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateDatasetRestriction", true);
    let api = DatasetRestrictionsAPI::with_config(configuration);
    let resp = api
        .update_dataset_restriction("product_type".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
