// Create a new dashboard with logs_stream list_stream widget and storage parameter
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
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
                                        ListStreamQuery::new(
                                            ListStreamSource::LOGS_STREAM,
                                            "".to_string(),
                                        ).storage("hot".to_string()),
                                        ListStreamResponseFormat::EVENT_LIST,
                                    )
                                ],
                                ListStreamWidgetDefinitionType::LIST_STREAM,
                            ),
                        ),
                    ),
                )
            ],
        );
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
