// Get a finding returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_security_monitoring::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetFinding", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp =
        api
            .get_finding(
                "AgAAAYd59gjghzF52gAAAAAAAAAYAAAAAEFZZDU5Z2pnQUFCRTRvV1lFeEo4SlFBQQAAACQAAAAAMDE4NzdhMDEtMDRiYS00NTZlLWFmMzMtNTIxNmNkNjVlNDMz".to_string(),
                GetFindingOptionalParams::default(),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
