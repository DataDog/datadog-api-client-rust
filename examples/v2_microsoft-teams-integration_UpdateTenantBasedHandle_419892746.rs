// Update api handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsTenantBasedHandleType;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateTenantBasedHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateTenantBasedHandleRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "tenant_based_handle" in the system
    let tenant_based_handle_data_id = std::env::var("TENANT_BASED_HANDLE_DATA_ID").unwrap();
    let body = MicrosoftTeamsUpdateTenantBasedHandleRequest::new(
        MicrosoftTeamsUpdateTenantBasedHandleRequestData::new(
            MicrosoftTeamsTenantBasedHandleAttributes::new()
                .name("fake-handle-name--updated".to_string()),
            MicrosoftTeamsTenantBasedHandleType::TENANT_BASED_HANDLE,
        ),
    );

    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_tenant_based_handle(tenant_based_handle_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
