// Create a new dashboard with list_stream widget with a valid sort parameter ASC
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::*;

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
                                        ListStreamQuery::new(ListStreamSource::EVENT_STREAM, "".to_string())
                                            .event_size(WidgetEventSize::LARGE)
                                            .sort(
                                                WidgetFieldSort::new("timestamp".to_string(), WidgetSort::ASCENDING),
                                            ),
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
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
