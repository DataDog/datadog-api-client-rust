// Get historical cost across your account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api::api_usage_metering::GetHistoricalCostByOrgOptionalParams;
use datadog_api_client::datadogV2::api::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_historical_cost_by_org(
            "2021-09-11T11:11:11+00:00".to_string(),
            GetHistoricalCostByOrgOptionalParams::default().view("sub-org".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
