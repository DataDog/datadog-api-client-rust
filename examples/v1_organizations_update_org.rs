// Update your organization returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_organizations::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = Organization::new()
        .billing(OrganizationBilling::new().type_("parent_billing".to_string()))
        .description("some description".to_string())
        .name("New child org".to_string())
        .public_id("abcdef12345".to_string())
        .settings(
            OrganizationSettings::new()
                .private_widget_share(false)
                .saml(OrganizationSettingsSaml::new().enabled(false))
                .saml_autocreate_access_role(Some(AccessRole::READ_ONLY))
                .saml_autocreate_users_domains(
                    OrganizationSettingsSamlAutocreateUsersDomains::new()
                        .domains(vec!["example.com".to_string()])
                        .enabled(false),
                )
                .saml_can_be_enabled(false)
                .saml_idp_endpoint("https://my.saml.endpoint".to_string())
                .saml_idp_initiated_login(
                    OrganizationSettingsSamlIdpInitiatedLogin::new().enabled(false),
                )
                .saml_idp_metadata_uploaded(false)
                .saml_login_url("https://my.saml.login.url".to_string())
                .saml_strict_mode(OrganizationSettingsSamlStrictMode::new().enabled(false)),
        )
        .subscription(OrganizationSubscription::new().type_("pro".to_string()))
        .trial(false);
    let configuration = Configuration::new();
    let api = OrganizationsAPI::with_config(configuration);
    let resp = api.update_org("abc123".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
