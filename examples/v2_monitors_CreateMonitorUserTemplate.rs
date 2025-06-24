// Create a monitor user template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;
use datadog_api_client::datadogV2::model::MonitorUserTemplateCreateData;
use datadog_api_client::datadogV2::model::MonitorUserTemplateCreateRequest;
use datadog_api_client::datadogV2::model::MonitorUserTemplateRequestAttributes;
use datadog_api_client::datadogV2::model::MonitorUserTemplateResourceType;
use datadog_api_client::datadogV2::model::MonitorUserTemplateTemplateVariablesItems;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = MonitorUserTemplateCreateRequest::new(MonitorUserTemplateCreateData::new(
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
        MonitorUserTemplateResourceType::MONITOR_USER_TEMPLATE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateMonitorUserTemplate", true);
    let api = MonitorsAPI::with_config(configuration);
    let resp = api.create_monitor_user_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
