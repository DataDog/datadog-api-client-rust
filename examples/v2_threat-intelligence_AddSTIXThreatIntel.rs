// Ingest STIX threat intelligence returns "OK" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_threat_intelligence::AddSTIXThreatIntelOptionalParams;
use datadog_api_client::datadogV2::api_threat_intelligence::ThreatIntelligenceAPI;
use datadog_api_client::datadogV2::model::STIXBundleRequest;
use datadog_api_client::datadogV2::model::STIXBundleType;
use datadog_api_client::datadogV2::model::STIXObject;
use datadog_api_client::datadogV2::model::STIXPatternType;
use datadog_api_client::datadogV2::model::STIXSpecVersion;

#[tokio::main]
async fn main() {
    let body = STIXBundleRequest::new(
        "bundle--44444444-4444-4444-8444-444444444444".to_string(),
        vec![STIXObject::new(
            "indicator--55555555-5555-4555-8555-555555555555".to_string(),
            "indicator".to_string(),
        )
        .created(
            DateTime::parse_from_rfc3339("2026-07-22T12:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .modified(
            DateTime::parse_from_rfc3339("2026-07-22T12:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .pattern("[ipv4-addr:value = '198.51.100.42']".to_string())
        .pattern_type(STIXPatternType::STIX)
        .spec_version("2.1".to_string())
        .valid_from(
            DateTime::parse_from_rfc3339("2026-07-22T12:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )],
        STIXBundleType::BUNDLE,
    )
    .spec_version(STIXSpecVersion::VERSION_2_1);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AddSTIXThreatIntel", true);
    let api = ThreatIntelligenceAPI::with_config(configuration);
    let resp = api
        .add_stix_threat_intel(
            "Acme-Inc".to_string(),
            body,
            AddSTIXThreatIntelOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
