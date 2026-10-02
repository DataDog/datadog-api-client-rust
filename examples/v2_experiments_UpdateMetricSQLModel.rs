// Update metric SQL model returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType;
use datadog_api_client::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricSQLModelV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricSQLModelV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems;
use datadog_api_client::datadogV2::model::ExperimentsMetricSQLModelPropertyInput;
use datadog_api_client::datadogV2::model::ExperimentsUpdateMetricSQLModelV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsUpdateMetricSQLModelV2RequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ExperimentsCreateMetricSQLModelV2Request::new(
        ExperimentsCreateMetricSQLModelV2RequestData::new(
            ExperimentsUpdateMetricSQLModelV2RequestDataAttributes::new(
                "Order facts".to_string(),
                "SELECT user_id, order_id, item_type, revenue, created_at FROM analytics.orders"
                    .to_string(),
                vec![
                    ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems::new(
                        "user_id".to_string(),
                        "550e8400-e29b-41d4-a716-446655440010".to_string(),
                    ),
                ],
                "created_at".to_string(),
            )
            .date_partition_column(None)
            .description(None)
            .measures(vec![
                ExperimentsCreateMetricSQLModelV2RequestDataAttributesMeasuresItems::new(
                    "revenue".to_string(),
                    ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType::FLOAT,
                )
                .description(None)
                .name(None),
            ])
            .properties(vec![ExperimentsMetricSQLModelPropertyInput::new(
                "item_type".to_string(),
                ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType::STRING,
                "item_type".to_string(),
            )
            .description(None)]),
            ExperimentsUpdateMetricSQLModelV2RequestDataType::METRIC_SQL_MODELS,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api
        .update_metric_sql_model(
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
