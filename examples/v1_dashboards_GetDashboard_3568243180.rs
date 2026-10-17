// Get a dashboard with five team tags and two AI tags
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard_with_team_and_ai_tags" in the system
    let dashboard_with_team_and_ai_tags_id =
        std::env::var("DASHBOARD_WITH_TEAM_AND_AI_TAGS_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api
        .get_dashboard(dashboard_with_team_and_ai_tags_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
