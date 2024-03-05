// Edit a monitor configuration policy returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_monitors::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "monitor_configuration_policy" in the system
    let monitor_configuration_policy_data_id = std::env::var("MONITOR_CONFIGURATION_POLICY_DATA_ID").unwrap();
    let body =
        MonitorConfigPolicyEditRequest::new(
            MonitorConfigPolicyEditData::new(
                MonitorConfigPolicyAttributeEditRequest::new(
                    MonitorConfigPolicyPolicy::MonitorConfigPolicyTagPolicy(
                        Box::new(
                            MonitorConfigPolicyTagPolicy::new()
                                .tag_key("examplemonitor".to_string())
                                .tag_key_required(false)
                                .valid_tag_values(vec!["prod".to_string(), "staging".to_string()]),
                        ),
                    ),
                    MonitorConfigPolicyType::TAG,
                ),
                monitor_configuration_policy_data_id,
                MonitorConfigPolicyResourceType::MONITOR_CONFIG_POLICY,
            ),
        );
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.update_monitor_config_policy(monitor_configuration_policy_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
