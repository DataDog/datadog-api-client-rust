// Create a new dashboard with funnel widget
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard with funnel widget".to_string(),
        vec![Widget::new(WidgetDefinition::FunnelWidgetDefinition(
            Box::new(FunnelWidgetDefinition::new(
                vec![FunnelWidgetRequest::new(
                    FunnelQuery::new(FunnelSource::RUM, "".to_string(), vec![]),
                    FunnelRequestType::FUNNEL,
                )],
                FunnelWidgetDefinitionType::FUNNEL,
            )),
        ))],
    );
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
