// Update App Favorite Status returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppFavoriteType;
use datadog_api_client::datadogV2::model::UpdateAppFavoriteRequest;
use datadog_api_client::datadogV2::model::UpdateAppFavoriteRequestData;
use datadog_api_client::datadogV2::model::UpdateAppFavoriteRequestDataAttributes;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpdateAppFavoriteRequest::new().data(
        UpdateAppFavoriteRequestData::new()
            .attributes(UpdateAppFavoriteRequestDataAttributes::new(true))
            .type_(AppFavoriteType::FAVORITES),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api
        .update_app_favorite(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
