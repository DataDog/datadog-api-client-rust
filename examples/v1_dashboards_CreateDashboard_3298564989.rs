// Create a new dashboard with llm_observability_stream list_stream widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::ListStreamColumn;
use datadog_api_client::datadogV1::model::ListStreamColumnWidth;
use datadog_api_client::datadogV1::model::ListStreamQuery;
use datadog_api_client::datadogV1::model::ListStreamResponseFormat;
use datadog_api_client::datadogV1::model::ListStreamSource;
use datadog_api_client::datadogV1::model::ListStreamWidgetDefinition;
use datadog_api_client::datadogV1::model::ListStreamWidgetDefinitionType;
use datadog_api_client::datadogV1::model::ListStreamWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard with list_stream widget".to_string(),
        vec![Widget::new(WidgetDefinition::ListStreamWidgetDefinition(
            Box::new(ListStreamWidgetDefinition::new(
                vec![ListStreamWidgetRequest::new(
                    vec![
                        ListStreamColumn::new(
                            "@status".to_string(),
                            ListStreamColumnWidth::COMPACT,
                        ),
                        ListStreamColumn::new(
                            "@content.prompt".to_string(),
                            ListStreamColumnWidth::AUTO,
                        ),
                        ListStreamColumn::new(
                            "@content.response.content".to_string(),
                            ListStreamColumnWidth::AUTO,
                        ),
                        ListStreamColumn::new("timestamp".to_string(), ListStreamColumnWidth::AUTO),
                        ListStreamColumn::new("@ml_app".to_string(), ListStreamColumnWidth::AUTO),
                        ListStreamColumn::new("service".to_string(), ListStreamColumnWidth::AUTO),
                        ListStreamColumn::new(
                            "@meta.evaluations.quality".to_string(),
                            ListStreamColumnWidth::AUTO,
                        ),
                        ListStreamColumn::new(
                            "@meta.evaluations.security".to_string(),
                            ListStreamColumnWidth::AUTO,
                        ),
                        ListStreamColumn::new("@duration".to_string(), ListStreamColumnWidth::AUTO),
                    ],
                    ListStreamQuery::new(
                        ListStreamSource::LLM_OBSERVABILITY_STREAM,
                        "@event_type:span @parent_id:undefined".to_string(),
                    )
                    .indexes(vec![]),
                    ListStreamResponseFormat::EVENT_LIST,
                )],
                ListStreamWidgetDefinitionType::LIST_STREAM,
            )),
        ))],
    );
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
