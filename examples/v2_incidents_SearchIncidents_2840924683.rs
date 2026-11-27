// Search for incidents accepts responders include
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::api_incidents::SearchIncidentsOptionalParams;
use datadog_api_client::datadogV2::model::IncidentRelatedObject;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.search_incidents", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .search_incidents(
            "state:(active OR stable OR resolved)".to_string(),
            SearchIncidentsOptionalParams::default()
                .page_size(1)
                .include(IncidentRelatedObject::RESPONDERS),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
