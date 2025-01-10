// Sets Domain Allowlist returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_domain_allowlist::DomainAllowlistAPI;
use datadog_api_client::datadogV2::model::DomainAllowlist;
use datadog_api_client::datadogV2::model::DomainAllowlistAttributes;
use datadog_api_client::datadogV2::model::DomainAllowlistRequest;
use datadog_api_client::datadogV2::model::DomainAllowlistType;

#[tokio::main]
async fn main() {
    let body = DomainAllowlistRequest::new(
        DomainAllowlist::new(DomainAllowlistType::DOMAIN_ALLOWLIST).attributes(
            DomainAllowlistAttributes::new()
                .domains(vec!["@static-test-domain.test".to_string()])
                .enabled(false),
        ),
    );

    let configuration = datadog::Configuration::new();
    let api = DomainAllowlistAPI::with_config(configuration);
    let resp = api.patch_domain_allowlist(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
