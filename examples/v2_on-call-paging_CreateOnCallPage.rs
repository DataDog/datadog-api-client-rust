// Create On-Call Page returns "OK." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call_paging::OnCallPagingAPI;
use datadog_api_client::datadogV2::model::CreatePageRequest;
use datadog_api_client::datadogV2::model::CreatePageRequestData;
use datadog_api_client::datadogV2::model::CreatePageRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreatePageRequestDataAttributesTarget;
use datadog_api_client::datadogV2::model::CreatePageRequestDataType;

#[tokio::main]
async fn main() {
    let body = CreatePageRequest::new().data(
        CreatePageRequestData::new(CreatePageRequestDataType::PAGES).attributes(
            CreatePageRequestDataAttributes::new(
                CreatePageRequestDataAttributesTarget::new()
                    .identifier("my-team".to_string())
                    .type_("team_handle".to_string()),
                "Page title".to_string(),
                "low".to_string(),
            )
            .description("Page details.".to_string())
            .tags(vec!["service:test".to_string()]),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallPagingAPI::with_config(configuration);
    let resp = api.create_on_call_page(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
