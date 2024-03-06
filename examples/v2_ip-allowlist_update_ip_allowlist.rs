// Update IP Allowlist returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ip_allowlist::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        IPAllowlistUpdateRequest::new(
            IPAllowlistData::new(
                IPAllowlistType::IP_ALLOWLIST,
            ).attributes(
                IPAllowlistAttributes::new()
                    .enabled(false)
                    .entries(
                        vec![
                            IPAllowlistEntry::new(
                                IPAllowlistEntryData::new(
                                    IPAllowlistEntryType::IP_ALLOWLIST_ENTRY,
                                ).attributes(
                                    IPAllowlistEntryAttributes::new()
                                        .cidr_block("127.0.0.1".to_string())
                                        .note("Example-IP-Allowlist".to_string()),
                                ),
                            )
                        ],
                    ),
            ),
        );
    let configuration = Configuration::new();
    let api = IPAllowlistAPI::with_config(configuration);
    let resp = api.update_ip_allowlist(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
