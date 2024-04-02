// Get cost across multi-org account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api::api_usage_metering::GetCostByOrgOptionalParams;
use datadog_api_client::datadogV2::api::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_cost_by_org(
            "2021-11-08T11:11:11+00:00".to_string(),
            GetCostByOrgOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
