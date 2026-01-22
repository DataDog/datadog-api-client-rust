// Validate budget returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::BudgetValidationRequest;
use datadog_api_client::datadogV2::model::BudgetValidationRequestData;
use datadog_api_client::datadogV2::model::BudgetWithEntriesDataAttributes;
use datadog_api_client::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItems;
use datadog_api_client::datadogV2::model::BudgetWithEntriesDataAttributesEntriesItemsTagFiltersItems;
use datadog_api_client::datadogV2::model::BudgetWithEntriesDataType;

#[tokio::main]
async fn main() {
    let body = BudgetValidationRequest::new().data(
        BudgetValidationRequestData::new(BudgetWithEntriesDataType::BUDGET)
            .attributes(
                BudgetWithEntriesDataAttributes::new()
                    .created_at(1738258683590)
                    .created_by("00000000-0a0a-0a0a-aaa0-00000000000a".to_string())
                    .end_month(202502)
                    .entries(vec![
                        BudgetWithEntriesDataAttributesEntriesItems::new()
                            .amount(500.0 as f64)
                            .month(202501)
                            .tag_filters(vec![
                                BudgetWithEntriesDataAttributesEntriesItemsTagFiltersItems::new()
                                    .tag_key("service".to_string())
                                    .tag_value("ec2".to_string()),
                            ]),
                        BudgetWithEntriesDataAttributesEntriesItems::new()
                            .amount(500.0 as f64)
                            .month(202502)
                            .tag_filters(vec![
                                BudgetWithEntriesDataAttributesEntriesItemsTagFiltersItems::new()
                                    .tag_key("service".to_string())
                                    .tag_value("ec2".to_string()),
                            ]),
                    ])
                    .metrics_query("aws.cost.amortized{service:ec2} by {service}".to_string())
                    .name("my budget".to_string())
                    .org_id(123)
                    .start_month(202501)
                    .total_amount(1000.0 as f64)
                    .updated_at(1738258683590)
                    .updated_by("00000000-0a0a-0a0a-aaa0-00000000000a".to_string()),
            )
            .id("1".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.validate_budget(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
