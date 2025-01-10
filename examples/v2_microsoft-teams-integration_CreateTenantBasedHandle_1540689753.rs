// Create api handle returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsCreateTenantBasedHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleRequestAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleRequestData;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleType;

#[tokio::main]
async fn main() {
    let body = MicrosoftTeamsCreateTenantBasedHandleRequest::new(
        MicrosoftTeamsTenantBasedHandleRequestData::new(
            MicrosoftTeamsTenantBasedHandleRequestAttributes::new(
                "19:iD_D2xy_sAa-JV851JJYwIa6mlW9F9Nxm3SLyZq68qY1@thread.tacv2".to_string(),
                "Example-Microsoft-Teams-Integration".to_string(),
                "e5f50a58-c929-4fb3-8866-e2cd836de3c2".to_string(),
                "4d3bac44-0230-4732-9e70-cc00736f0a97".to_string(),
            ),
            MicrosoftTeamsTenantBasedHandleType::TENANT_BASED_HANDLE,
        ),
    );

    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api.create_tenant_based_handle(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
