// Create a fleet access scope returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_fleet_automation::FleetAutomationAPI;
use datadog_api_client::datadogV2::model::FleetAccessScopeAttributes;
use datadog_api_client::datadogV2::model::FleetAccessScopeFeatures;
use datadog_api_client::datadogV2::model::FleetAccessScopeResourceType;
use datadog_api_client::datadogV2::model::FleetAccessScopeWrite;
use datadog_api_client::datadogV2::model::FleetAccessScopeWriteRequest;

#[tokio::main]
async fn main() {
    let body = FleetAccessScopeWriteRequest::new(
        FleetAccessScopeWrite::new(FleetAccessScopeResourceType::ACCESS_SCOPE).attributes(
            FleetAccessScopeAttributes::new()
                .description("Example-Fleet-Automation".to_string())
                .features(
                    FleetAccessScopeFeatures::new()
                        .configure(false)
                        .install(false)
                        .upgrade(true),
                )
                .query("env:prod".to_string()),
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_fleet_access_scope", true);
    let api = FleetAutomationAPI::with_config(configuration);
    let resp = api.create_fleet_access_scope(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
