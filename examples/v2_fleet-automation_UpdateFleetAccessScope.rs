// Replace a fleet access scope returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetAccessScopeAttributes;
use datadog_api_client::datadogV2::model::FleetAccessScopeFeatures;
use datadog_api_client::datadogV2::model::FleetAccessScopeResourceType;
use datadog_api_client::datadogV2::model::FleetAccessScopeWrite;
use datadog_api_client::datadogV2::model::FleetAccessScopeWriteRequest;

#[tokio::main]
async fn main() {
    // there is a valid "fleet_access_scope" in the system
    let fleet_access_scope_id = std::env::var("FLEET_ACCESS_SCOPE_ID").unwrap();
    let body = FleetAccessScopeWriteRequest::new(
        FleetAccessScopeWrite::new(FleetAccessScopeResourceType::ACCESS_SCOPE).attributes(
            FleetAccessScopeAttributes::new()
                .description("Example-Fleet-Automation updated".to_string())
                .features(
                    FleetAccessScopeFeatures::new()
                        .configure(true)
                        .install(false)
                        .upgrade(false),
                )
                .query("env:staging".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_fleet_access_scope", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api
        .update_fleet_access_scope(fleet_access_scope_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
