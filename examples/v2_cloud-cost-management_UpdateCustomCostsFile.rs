// Update Custom Costs file returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::CustomCostsFileUpdateAttributes;
use datadog_api_client::datadogV2::model::CustomCostsFileUpdateData;
use datadog_api_client::datadogV2::model::CustomCostsFileUpdateRequest;
use datadog_api_client::datadogV2::model::CustomCostsFileUpdateType;

#[tokio::main]
async fn main() {
    // there is a valid "custom_costs_file" in the system
    let custom_costs_file_data_id = std::env::var("CUSTOM_COSTS_FILE_DATA_ID").unwrap();
    let body = CustomCostsFileUpdateRequest::new(CustomCostsFileUpdateData::new(
        CustomCostsFileUpdateAttributes::new(true),
        custom_costs_file_data_id.clone(),
        CustomCostsFileUpdateType::COST_METADATA,
    ));
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_custom_costs_file(custom_costs_file_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
