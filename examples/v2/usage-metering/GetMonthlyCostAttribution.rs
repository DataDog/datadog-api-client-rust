// Get Monthly Cost Attribution returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::UsageMeteringAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetMonthlyCostAttribution", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api.get_monthly_cost_attribution().await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
