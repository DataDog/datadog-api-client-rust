// Create a unit cost returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::UnitCostCreateRequest;
use datadog_api_client::datadogV2::model::UnitCostCreateRequestData;
use datadog_api_client::datadogV2::model::UnitCostQueryDefinition;
use datadog_api_client::datadogV2::model::UnitCostRequestAttributes;
use datadog_api_client::datadogV2::model::UnitCostType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        UnitCostCreateRequest::new(
            UnitCostCreateRequestData::new(
                UnitCostRequestAttributes::new(
                    UnitCostQueryDefinition::new(
                        vec![BTreeMap::from([("formula".to_string(), Value::from("numerator"))])],
                        vec![
                            BTreeMap::from(
                                [
                                    ("data_source".to_string(), Value::from("cloud_cost")),
                                    ("name".to_string(), Value::from("numerator")),
                                    (
                                        "query".to_string(),
                                        Value::from(
                                            "sum:aws.cost.net.amortized.shared.resources.allocated{*}.rollup(sum, daily)",
                                        ),
                                    ),
                                ],
                            )
                        ],
                    ),
                    "Cloud cost per active user".to_string(),
                    UnitCostQueryDefinition::new(
                        vec![BTreeMap::from([("formula".to_string(), Value::from("numerator"))])],
                        vec![
                            BTreeMap::from(
                                [
                                    ("data_source".to_string(), Value::from("cloud_cost")),
                                    ("name".to_string(), Value::from("numerator")),
                                    (
                                        "query".to_string(),
                                        Value::from(
                                            "sum:aws.cost.net.amortized.shared.resources.allocated{*}.rollup(sum, daily)",
                                        ),
                                    ),
                                ],
                            )
                        ],
                    ),
                    "user".to_string(),
                ).description(Some("Amortized cloud spend divided by the number of active users.".to_string())),
                UnitCostType::UNIT_COST,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateUnitCost", true);
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.create_unit_cost(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
