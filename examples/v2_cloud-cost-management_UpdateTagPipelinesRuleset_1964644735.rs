// Update tag pipeline ruleset with if_tag_exists returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::DataAttributesRulesItemsIfTagExists;
use datadog_api_client::datadogV2::model::DataAttributesRulesItemsMapping;
use datadog_api_client::datadogV2::model::UpdateRulesetRequest;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestData;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataAttributes;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItems;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataType;

#[tokio::main]
async fn main() {
    let body = UpdateRulesetRequest::new().data(
        UpdateRulesetRequestData::new(UpdateRulesetRequestDataType::UPDATE_RULESET)
            .attributes(
                UpdateRulesetRequestDataAttributes::new(
                    true,
                    vec![UpdateRulesetRequestDataAttributesRulesItems::new(
                        true,
                        "Account Name Mapping".to_string(),
                    )
                    .mapping(Some(
                        DataAttributesRulesItemsMapping::new(
                            "team_owner".to_string(),
                            vec!["account_name".to_string(), "account_id".to_string()],
                        )
                        .if_tag_exists(DataAttributesRulesItemsIfTagExists::REPLACE),
                    ))
                    .query(None)
                    .reference_table(None)],
                )
                .last_version(3611102),
            )
            .id("New Ruleset".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_tag_pipelines_ruleset("ee10c3ff-312f-464c-b4f6-46adaa6d00a1".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
