// Get SLO status returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_level_objectives::GetSloStatusOptionalParams;
use datadog_api_client::datadogV2::api_service_level_objectives::ServiceLevelObjectivesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetSloStatus", true);
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api
        .get_slo_status(
            "00000000-0000-0000-0000-000000000000".to_string(),
            1690901870,
            1706803070,
            GetSloStatusOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
