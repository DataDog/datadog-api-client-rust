// GetEstimatedCostByOrg with start_month returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::GetEstimatedCostByOrgOptionalParams;
use datadog_api_client::datadogV2::api::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_estimated_cost_by_org(
            GetEstimatedCostByOrgOptionalParams::default()
                .view("sub-org".to_string())
                .start_month("2021-11-11T11:11:11+00:00".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
