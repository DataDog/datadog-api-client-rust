// Get a monitor user template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_monitors::GetMonitorUserTemplateOptionalParams;
use datadog_api_client::datadogV2::api_monitors::MonitorsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "monitor_user_template" in the system
    let monitor_user_template_data_id = std::env::var("MONITOR_USER_TEMPLATE_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetMonitorUserTemplate", true);
    let api = MonitorsAPI::with_config(configuration);
    let resp = api
        .get_monitor_user_template(
            monitor_user_template_data_id.clone(),
            GetMonitorUserTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
