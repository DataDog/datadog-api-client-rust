// Create status page returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateStatusPageOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateStatusPageRequest;
use datadog_api_client::datadogV2::model::CreateStatusPageRequestData;
use datadog_api_client::datadogV2::model::CreateStatusPageRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateStatusPageRequestDataAttributesType;
use datadog_api_client::datadogV2::model::CreateStatusPageRequestDataAttributesVisualizationType;
use datadog_api_client::datadogV2::model::StatusPageDataType;

#[tokio::main]
async fn main() {
    let body = CreateStatusPageRequest::new().data(
        CreateStatusPageRequestData::new(StatusPageDataType::STATUS_PAGES).attributes(
            CreateStatusPageRequestDataAttributes::new(
                "dd-integrations-tests-5e2fd69be33e79aa".to_string(),
                true,
                "[DD Integration Tests] 5e2fd69be33e79aa".to_string(),
                CreateStatusPageRequestDataAttributesType::INTERNAL,
                CreateStatusPageRequestDataAttributesVisualizationType::BARS_AND_UPTIME_PERCENTAGE,
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_status_page(body, CreateStatusPageOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
