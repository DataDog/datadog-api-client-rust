// Import vulnerabilities returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CycloneDXAdvisory;
use datadog_api_client::datadogV2::model::CycloneDXAffect;
use datadog_api_client::datadogV2::model::CycloneDXAssetComponent;
use datadog_api_client::datadogV2::model::CycloneDXBOM;
use datadog_api_client::datadogV2::model::CycloneDXComponent;
use datadog_api_client::datadogV2::model::CycloneDXComponentType;
use datadog_api_client::datadogV2::model::CycloneDXMetadata;
use datadog_api_client::datadogV2::model::CycloneDXRating;
use datadog_api_client::datadogV2::model::CycloneDXReference;
use datadog_api_client::datadogV2::model::CycloneDXReferenceSource;
use datadog_api_client::datadogV2::model::CycloneDXToolComponent;
use datadog_api_client::datadogV2::model::CycloneDXTools;
use datadog_api_client::datadogV2::model::CycloneDXVulnerability;

#[tokio::main]
async fn main() {
    let body =
        CycloneDXBOM::new(
            "CycloneDX".to_string(),
            vec![CycloneDXComponent::new(
                "a3390fca-c315-41ae-ae05-af5e7859cdee".to_string(),
                "lodash".to_string(),
                CycloneDXComponentType::LIBRARY,
                "4.17.21".to_string(),
            )
            .purl("pkg:npm/lodash@4.17.21".to_string())],
            CycloneDXMetadata::new(
                CycloneDXAssetComponent::new("i-12345".to_string())
                    .bom_ref("asset-ref-123".to_string())
                    .type_("operating-system".to_string()),
                CycloneDXTools::new(vec![CycloneDXToolComponent::new("my-scanner".to_string())
                    .type_("application".to_string())]),
            ),
            "1.5".to_string(),
            1,
            vec![CycloneDXVulnerability::new(
                vec![CycloneDXAffect::new(
                    "a3390fca-c315-41ae-ae05-af5e7859cdee".to_string(),
                )],
                "CVE-2021-1234".to_string(),
                vec![CycloneDXRating::new()
                    .score(9.0 as f64)
                    .severity("high".to_string())
                    .vector("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:N".to_string())],
            )
            .advisories(vec![CycloneDXAdvisory::new()
                .url("https://example.com/advisory/CVE-2021-1234".to_string())])
            .cwes(vec![123, 345])
            .description("Sample vulnerability detected in the application.".to_string())
            .detail("Details about the vulnerability".to_string())
            .references(vec![CycloneDXReference::new()
                .id("GHSA-35m5-8cvj-8783".to_string())
                .source(CycloneDXReferenceSource::new().url("https://example.com".to_string()))])],
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ImportSecurityVulnerabilities", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.import_security_vulnerabilities(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
