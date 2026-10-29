// Get service repository information returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum::RUMAPI;
use datadog_api_client::datadogV2::model::ServiceRepositoryInfoDataType;
use datadog_api_client::datadogV2::model::ServiceRepositoryInfoRequest;
use datadog_api_client::datadogV2::model::ServiceRepositoryInfoRequestAttributes;
use datadog_api_client::datadogV2::model::ServiceRepositoryInfoRequestData;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = ServiceRepositoryInfoRequest::new(ServiceRepositoryInfoRequestData::new(
        ServiceRepositoryInfoRequestAttributes::new().debug_id(
            Uuid::parse_str("00000000-0000-4000-8000-000000000001").expect("invalid UUID"),
        ),
        ServiceRepositoryInfoDataType::SERVICE_REPOSITORY_INFO,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_service_repository_info", true);
    let api = RUMAPI::with_config(configuration);
    let resp = api.get_service_repository_info(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
