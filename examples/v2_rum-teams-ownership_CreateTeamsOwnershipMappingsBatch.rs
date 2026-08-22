// Bulk create and remove teams ownership mappings returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_teams_ownership::RumTeamsOwnershipAPI;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingBatchOperation;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingBatchOperationData;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingBatchOperationDataAttributes;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingBatchOperationOp;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingBatchRequest;
use datadog_api_client::datadogV2::model::TeamsOwnershipMappingType;
use datadog_api_client::datadogV2::model::TeamsOwnershipMatchType;

#[tokio::main]
async fn main() {
    let body =
        TeamsOwnershipMappingBatchRequest::new(vec![TeamsOwnershipMappingBatchOperation::new(
            TeamsOwnershipMappingBatchOperationOp::ADD,
        )
        .data(TeamsOwnershipMappingBatchOperationData::new(
            TeamsOwnershipMappingBatchOperationDataAttributes::new()
                .match_type(TeamsOwnershipMatchType::EXACT)
                .service("web-checkout-examplerumteamsownership".to_string())
                .team_handle("team-rum".to_string())
                .view_name("/checkout-examplerumteamsownership".to_string()),
            TeamsOwnershipMappingType::TEAMS_OWNERSHIP_MAPPINGS,
        ))]);
    let configuration = datadog::Configuration::new();
    let api = RumTeamsOwnershipAPI::with_config(configuration);
    let resp = api.create_teams_ownership_mappings_batch(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
