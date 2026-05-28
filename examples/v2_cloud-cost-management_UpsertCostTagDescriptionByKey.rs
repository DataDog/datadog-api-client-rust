// Upsert a Cloud Cost Management tag description returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CostTagDescriptionType;
use datadog_api_client::datadogV2::model::CostTagDescriptionUpsertRequest;
use datadog_api_client::datadogV2::model::CostTagDescriptionUpsertRequestData;
use datadog_api_client::datadogV2::model::CostTagDescriptionUpsertRequestDataAttributes;

#[tokio::main]
async fn main() {
    let body = CostTagDescriptionUpsertRequest::new(
        CostTagDescriptionUpsertRequestData::new(
            CostTagDescriptionUpsertRequestDataAttributes::new(
                "AWS account that owns this cost.".to_string(),
            )
            .cloud("aws".to_string()),
            CostTagDescriptionType::COST_TAG_DESCRIPTION,
        )
        .id("account_id".to_string()),
    );
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .upsert_cost_tag_description_by_key("tag_key".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
