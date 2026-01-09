// Create component returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateComponentOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateComponentRequest;
use datadog_api_client::datadogV2::model::CreateComponentRequestData;
use datadog_api_client::datadogV2::model::CreateComponentRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateComponentRequestDataAttributesType;
use datadog_api_client::datadogV2::model::StatusPagesComponentGroupType;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = std::env::var("STATUS_PAGE_DATA_ID").unwrap();
    let body = CreateComponentRequest::new().data(
        CreateComponentRequestData::new(StatusPagesComponentGroupType::COMPONENTS).attributes(
            CreateComponentRequestDataAttributes::new(
                "5e2fd69be33e79aa".to_string(),
                0,
                CreateComponentRequestDataAttributesType::COMPONENT,
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_component(
            status_page_data_id.clone(),
            body,
            CreateComponentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
