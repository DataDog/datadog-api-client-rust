// Get hourly usage for CSM Pro returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_usage_metering::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = UsageMeteringAPI::with_config(configuration);
    let resp =
        api
            .get_usage_cloud_security_posture_management(
                (Utc::now() + chrono::Duration::days(-3)).to_rfc3339(),
                GetUsageCloudSecurityPostureManagementOptionalParams::default(),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
