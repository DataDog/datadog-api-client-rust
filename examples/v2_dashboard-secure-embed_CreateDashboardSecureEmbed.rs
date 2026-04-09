// Create a secure embed for a dashboard returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboard_secure_embed::DashboardSecureEmbedAPI;
use datadog_api_client::datadogV2::model::SecureEmbedCreateRequest;
use datadog_api_client::datadogV2::model::SecureEmbedCreateRequestAttributes;
use datadog_api_client::datadogV2::model::SecureEmbedCreateRequestData;
use datadog_api_client::datadogV2::model::SecureEmbedGlobalTime;
use datadog_api_client::datadogV2::model::SecureEmbedGlobalTimeLiveSpan;
use datadog_api_client::datadogV2::model::SecureEmbedRequestType;
use datadog_api_client::datadogV2::model::SecureEmbedSelectableTemplateVariable;
use datadog_api_client::datadogV2::model::SecureEmbedStatus;
use datadog_api_client::datadogV2::model::SecureEmbedViewingPreferences;
use datadog_api_client::datadogV2::model::SecureEmbedViewingPreferencesTheme;

#[tokio::main]
async fn main() {
    let body = SecureEmbedCreateRequest::new(SecureEmbedCreateRequestData::new(
        SecureEmbedCreateRequestAttributes::new(
            SecureEmbedGlobalTime::new().live_span(SecureEmbedGlobalTimeLiveSpan::PAST_ONE_HOUR),
            true,
            vec![SecureEmbedSelectableTemplateVariable::new()
                .default_values(vec!["1".to_string()])
                .name("org_id".to_string())
                .prefix("org_id".to_string())
                .visible_tags(vec!["1".to_string()])],
            SecureEmbedStatus::ACTIVE,
            "Q1 Metrics Dashboard".to_string(),
            SecureEmbedViewingPreferences::new()
                .high_density(false)
                .theme(SecureEmbedViewingPreferencesTheme::SYSTEM),
        ),
        SecureEmbedRequestType::SECURE_EMBED_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDashboardSecureEmbed", true);
    let api = DashboardSecureEmbedAPI::with_config(configuration);
    let resp = api
        .create_dashboard_secure_embed("dashboard_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
