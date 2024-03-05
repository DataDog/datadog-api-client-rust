// Get Monthly Cost Attribution returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_usage_metering::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetMonthlyCostAttribution", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp =
        api
            .get_monthly_cost_attribution(
                (Utc::now() + chrono::Duration::days(-5)).to_rfc3339(),
                (Utc::now() + chrono::Duration::days(-3)).to_rfc3339(),
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
