// Create tag pipeline ruleset returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CreateRulesetRequest;
use datadog_api_client::datadogV2::model::CreateRulesetRequestData;
use datadog_api_client::datadogV2::model::CreateRulesetRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateRulesetRequestDataAttributesRulesItems;
use datadog_api_client::datadogV2::model::CreateRulesetRequestDataAttributesRulesItemsQuery;
use datadog_api_client::datadogV2::model::CreateRulesetRequestDataAttributesRulesItemsQueryAddition;
use datadog_api_client::datadogV2::model::CreateRulesetRequestDataType;

#[tokio::main]
async fn main() {
    let body = CreateRulesetRequest::new().data(
        CreateRulesetRequestData::new(CreateRulesetRequestDataType::CREATE_RULESET)
            .attributes(
                CreateRulesetRequestDataAttributes::new(vec![
                    CreateRulesetRequestDataAttributesRulesItems::new(
                        true,
                        "Add Cost Center Tag".to_string(),
                    )
                    .mapping(None)
                    .query(Some(
                        CreateRulesetRequestDataAttributesRulesItemsQuery::new(
                            Some(
                                CreateRulesetRequestDataAttributesRulesItemsQueryAddition::new(
                                    "cost_center".to_string(),
                                    "engineering".to_string(),
                                ),
                            ),
                            true,
                            r#"account_id:"123456789" AND service:"web-api""#.to_string(),
                        )
                        .case_insensitivity(false),
                    ))
                    .reference_table(None),
                ])
                .enabled(true),
            )
            .id("New Ruleset".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.create_tag_pipelines_ruleset(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
