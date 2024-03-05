// Update a dashboard with tags returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard" in the system
    let dashboard_id = std::env::var("DASHBOARD_ID").unwrap();
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard with list_stream widget".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::ListStreamWidgetDefinition(
                        Box::new(
                            ListStreamWidgetDefinition::new(
                                vec![
                                    ListStreamWidgetRequest::new(
                                        vec![
                                            ListStreamColumn::new("timestamp".to_string(), ListStreamColumnWidth::AUTO)
                                        ],
                                        ListStreamQuery::new(ListStreamSource::APM_ISSUE_STREAM, "".to_string()),
                                        ListStreamResponseFormat::EVENT_LIST,
                                    )
                                ],
                                ListStreamWidgetDefinitionType::LIST_STREAM,
                            ),
                        ),
                    ),
                )
            ],
        )
            .description(Some("Updated description".to_string()))
            .tags(Some(vec!["team:foo".to_string(), "team:bar".to_string()]));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.update_dashboard(dashboard_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
