// Update ruleset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::UpdateRulesetRequest;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestData;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataAttributes;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItems;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataAttributesRulesItemsMapping;
use datadog_api_client::datadogV2::model::UpdateRulesetRequestDataType;

#[tokio::main]
async fn main() {
    let body = UpdateRulesetRequest::new().data(
        UpdateRulesetRequestData::new(UpdateRulesetRequestDataType::UPDATE_RULESET).attributes(
            UpdateRulesetRequestDataAttributes::new(
                true,
                vec![UpdateRulesetRequestDataAttributesRulesItems::new(
                    true,
                    "Account Name Mapping".to_string(),
                )
                .mapping(Some(
                    UpdateRulesetRequestDataAttributesRulesItemsMapping::new(
                        "team_owner".to_string(),
                        true,
                        vec!["account_name".to_string(), "account_id".to_string()],
                    ),
                ))
                .query(None)
                .reference_table(None)],
            )
            .last_version(3601919),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_ruleset("1c5dae14-237d-4b9a-a515-aa55b3939142".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
