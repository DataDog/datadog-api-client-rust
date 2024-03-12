// Get Monthly Cost Attribution returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::*;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetMonthlyCostAttribution", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .get_monthly_cost_attribution(
            "2021-11-06T11:11:11+00:00".to_string(),
            "2021-11-08T11:11:11+00:00".to_string(),
            "infra_host_total_cost".to_string(),
            GetMonthlyCostAttributionOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
