// Update Cloud Cost Management GCP Usage Cost config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPatchData;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPatchRequest;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPatchRequestAttributes;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPatchRequestType;

#[tokio::main]
async fn main() {
    let body = GCPUsageCostConfigPatchRequest::new(GCPUsageCostConfigPatchData::new(
        GCPUsageCostConfigPatchRequestAttributes::new(true),
        GCPUsageCostConfigPatchRequestType::GCP_USAGE_COST_CONFIG_PATCH_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_cost_gcp_usage_cost_config("100".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
