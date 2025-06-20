// Validate an existing monitor user template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV2::model::MonitorUserTemplateRequestAttributes;
use datadog_api_client::datadogV2::model::MonitorUserTemplateResourceType;
use datadog_api_client::datadogV2::model::MonitorUserTemplateTemplateVariablesItems;
use datadog_api_client::datadogV2::model::MonitorUserTemplateUpdateData;
use datadog_api_client::datadogV2::model::MonitorUserTemplateUpdateRequest;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "monitor_user_template" in the system
    let monitor_user_template_data_id = std::env::var("MONITOR_USER_TEMPLATE_DATA_ID").unwrap();
    let body = MonitorUserTemplateUpdateRequest::new(MonitorUserTemplateUpdateData::new(
        MonitorUserTemplateRequestAttributes::new(
            BTreeMap::from([
                ("message".to_string(), Value::from("A msg.")),
                ("name".to_string(), Value::from("A name example-monitor")),
                (
                    "query".to_string(),
                    Value::from("avg(last_5m):sum:system.net.bytes_rcvd{host:host0} > 100"),
                ),
                ("type".to_string(), Value::from("query alert")),
            ]),
            vec!["integration:Azure".to_string()],
            "Postgres DB example-monitor".to_string(),
        )
        .description(Some("A description.".to_string()))
        .template_variables(vec![MonitorUserTemplateTemplateVariablesItems::new(
            "regionName".to_string(),
        )
        .available_values(vec!["value1".to_string(), "value2".to_string()])
        .defaults(vec!["defaultValue".to_string()])
        .tag_key("datacenter".to_string())]),
        monitor_user_template_data_id.clone(),
        MonitorUserTemplateResourceType::MONITOR_USER_TEMPLATE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ValidateExistingMonitorUserTemplate", true);
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .validate_existing_monitor_user_template(monitor_user_template_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
