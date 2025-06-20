// Create Cloud Cost Management GCP Usage Cost config returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPostData;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPostRequest;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPostRequestAttributes;
use datadog_api_client::datadogV2::model::GCPUsageCostConfigPostRequestType;

#[tokio::main]
async fn main() {
    let body = GCPUsageCostConfigPostRequest::new(GCPUsageCostConfigPostData::new(
        GCPUsageCostConfigPostRequestAttributes::new(
            "123456_A123BC_12AB34".to_string(),
            "dd-cost-bucket".to_string(),
            "billing".to_string(),
            "dd-cloud-cost-report".to_string(),
            "dd-ccm-gcp-integration@my-environment.iam.gserviceaccount.com".to_string(),
        )
        .export_prefix("datadog_cloud_cost_usage_export".to_string()),
        GCPUsageCostConfigPostRequestType::GCP_USAGE_COST_CONFIG_POST_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api.create_cost_gcp_usage_cost_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
