// Update arbitrary cost rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequest;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequestData;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequestDataAttributes;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesCostsToAllocateItems;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategy;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequestDataAttributesStrategyBasedOnCostsItems;
use datadog_api_client::datadogV2::model::ArbitraryCostUpsertRequestDataType;

#[tokio::main]
async fn main() {
    let body = ArbitraryCostUpsertRequest::new().data(
        ArbitraryCostUpsertRequestData::new(
            ArbitraryCostUpsertRequestDataType::UPSERT_ARBITRARY_RULE,
        )
        .attributes(
            ArbitraryCostUpsertRequestDataAttributes::new(
                vec![
                    ArbitraryCostUpsertRequestDataAttributesCostsToAllocateItems::new(
                        "is".to_string(),
                        "account_id".to_string(),
                    )
                    .value("123456789".to_string())
                    .values(Some(vec![])),
                    ArbitraryCostUpsertRequestDataAttributesCostsToAllocateItems::new(
                        "in".to_string(),
                        "environment".to_string(),
                    )
                    .value("".to_string())
                    .values(Some(vec!["production".to_string(), "staging".to_string()])),
                ],
                vec!["aws".to_string(), "gcp".to_string()],
                "example-arbitrary-cost-rule".to_string(),
                ArbitraryCostUpsertRequestDataAttributesStrategy::new("proportional".to_string())
                    .allocated_by_tag_keys(vec!["team".to_string(), "environment".to_string()])
                    .based_on_costs(vec![
                        ArbitraryCostUpsertRequestDataAttributesStrategyBasedOnCostsItems::new(
                            "is".to_string(),
                            "service".to_string(),
                        )
                        .value("web-api".to_string())
                        .values(Some(vec![])),
                        ArbitraryCostUpsertRequestDataAttributesStrategyBasedOnCostsItems::new(
                            "not in".to_string(),
                            "team".to_string(),
                        )
                        .value("".to_string())
                        .values(Some(vec!["legacy".to_string(), "deprecated".to_string()])),
                    ])
                    .granularity("daily".to_string()),
                "shared".to_string(),
            )
            .enabled(true)
            .order_id(1),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.update_arbitrary_cost_rule(123456, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
