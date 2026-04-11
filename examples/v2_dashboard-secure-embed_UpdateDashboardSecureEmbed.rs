// Update a secure embed for a dashboard returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboard_secure_embed::DashboardSecureEmbedAPI;
use datadog_api_client::datadogV2::model::SecureEmbedGlobalTime;
use datadog_api_client::datadogV2::model::SecureEmbedGlobalTimeLiveSpan;
use datadog_api_client::datadogV2::model::SecureEmbedSelectableTemplateVariable;
use datadog_api_client::datadogV2::model::SecureEmbedStatus;
use datadog_api_client::datadogV2::model::SecureEmbedUpdateRequest;
use datadog_api_client::datadogV2::model::SecureEmbedUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::SecureEmbedUpdateRequestData;
use datadog_api_client::datadogV2::model::SecureEmbedUpdateRequestType;
use datadog_api_client::datadogV2::model::SecureEmbedViewingPreferences;
use datadog_api_client::datadogV2::model::SecureEmbedViewingPreferencesTheme;

#[tokio::main]
async fn main() {
    let body = SecureEmbedUpdateRequest::new(SecureEmbedUpdateRequestData::new(
        SecureEmbedUpdateRequestAttributes::new()
            .global_time(
                SecureEmbedGlobalTime::new()
                    .live_span(SecureEmbedGlobalTimeLiveSpan::PAST_ONE_HOUR),
            )
            .global_time_selectable(true)
            .selectable_template_vars(vec![SecureEmbedSelectableTemplateVariable::new()
                .default_values(vec!["1".to_string()])
                .name("org_id".to_string())
                .prefix("org_id".to_string())
                .visible_tags(vec!["1".to_string()])])
            .status(SecureEmbedStatus::ACTIVE)
            .title("Q1 Metrics Dashboard (Updated)".to_string())
            .viewing_preferences(
                SecureEmbedViewingPreferences::new()
                    .high_density(false)
                    .theme(SecureEmbedViewingPreferencesTheme::SYSTEM),
            ),
        SecureEmbedUpdateRequestType::SECURE_EMBED_UPDATE_REQUEST,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateDashboardSecureEmbed", true);
    let api = DashboardSecureEmbedAPI::with_config(configuration);
    let resp = api
        .update_dashboard_secure_embed("dashboard_id".to_string(), "token".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
