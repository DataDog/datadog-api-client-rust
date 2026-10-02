// Create exposure SQL model returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateExposureSQLModelV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType;
use datadog_api_client::datadogV2::model::ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems;
use datadog_api_client::datadogV2::model::ExperimentsSQLModelPropertyInput;
use datadog_api_client::datadogV2::model::ExperimentsUpdateExposureSQLModelV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsUpdateExposureSQLModelV2RequestDataType;

#[tokio::main]
async fn main() {
    let body =
        ExperimentsCreateExposureSQLModelV2Request::new(
            ExperimentsCreateExposureSQLModelV2RequestData::new(
                ExperimentsUpdateExposureSQLModelV2RequestDataAttributes::new(
                    "experiment_id".to_string(),
                    "Exposure events".to_string(),
                    "SELECT user_id, experiment_id, variant, exposed_at, country FROM analytics.exposures".to_string(),
                    vec![
                        ExperimentsCreateExposureSQLModelV2RequestDataAttributesSubjectTypesItems::new(
                            "user_id".to_string(),
                            "550e8400-e29b-41d4-a716-446655440010".to_string(),
                        )
                    ],
                    "exposed_at".to_string(),
                    "variant".to_string(),
                )
                    .date_partition_column(None)
                    .properties(
                        vec![
                            ExperimentsSQLModelPropertyInput::new("country".to_string(), "country".to_string())
                                .column_type(
                                    ExperimentsCreateExposureSQLModelV2RequestDataAttributesItemsColumnType::STRING,
                                )
                                .description(None)
                        ],
                    ),
                ExperimentsUpdateExposureSQLModelV2RequestDataType::EXPOSURE_SQL_MODELS,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_exposure_sql_model(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
