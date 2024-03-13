// Create a monitor configuration policy returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_monitors::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body = MonitorConfigPolicyCreateRequest::new(MonitorConfigPolicyCreateData::new(
        MonitorConfigPolicyAttributeCreateRequest::new(
            MonitorConfigPolicyPolicyCreateRequest::MonitorConfigPolicyTagPolicyCreateRequest(
                Box::new(MonitorConfigPolicyTagPolicyCreateRequest::new(
                    "examplemonitor".to_string(),
                    false,
                    vec!["prod".to_string(), "staging".to_string()],
                )),
            ),
            MonitorConfigPolicyType::TAG,
        ),
        MonitorConfigPolicyResourceType::MONITOR_CONFIG_POLICY,
    ));
    let configuration = Configuration::new();
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor_config_policy(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
