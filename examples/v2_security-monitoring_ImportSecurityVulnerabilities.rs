// Import security vulnerabilities returns "Vulnerabilities accepted
// successfully." response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::CycloneDXBom;
use datadog_api_client::datadogV2::model::CycloneDXComponent;
use datadog_api_client::datadogV2::model::CycloneDXComponentType;
use datadog_api_client::datadogV2::model::CycloneDXMetadata;
use datadog_api_client::datadogV2::model::CycloneDXMetadataComponent;
use datadog_api_client::datadogV2::model::CycloneDXMetadataTools;
use datadog_api_client::datadogV2::model::CycloneDXToolComponent;
use datadog_api_client::datadogV2::model::CycloneDXVulnerability;
use datadog_api_client::datadogV2::model::CycloneDXVulnerabilityAdvisory;
use datadog_api_client::datadogV2::model::CycloneDXVulnerabilityAffects;
use datadog_api_client::datadogV2::model::CycloneDXVulnerabilityAnalysis;
use datadog_api_client::datadogV2::model::CycloneDXVulnerabilityRating;
use datadog_api_client::datadogV2::model::CycloneDXVulnerabilityReference;
use datadog_api_client::datadogV2::model::CycloneDXVulnerabilityReferenceSource;

#[tokio::main]
async fn main() {
    let body = CycloneDXBom::new(
        "CycloneDX".to_string(),
        vec![CycloneDXComponent::new(
            "a3390fca-c315-41ae-ae05-af5e7859cdee".to_string(),
            "lodash".to_string(),
            CycloneDXComponentType::LIBRARY,
            "4.17.21".to_string(),
        )
        .purl("pkg:npm/lodash@4.17.21".to_string())],
        CycloneDXMetadata::new(
            CycloneDXMetadataComponent::new("i-12345".to_string())
                .bom_ref("host-ref-abc123".to_string())
                .type_("operating-system".to_string()),
            CycloneDXMetadataTools::new(vec![CycloneDXToolComponent::new(
                "my-scanner".to_string(),
            )
            .type_("application".to_string())]),
        ),
        "1.5".to_string(),
        vec![CycloneDXVulnerability::new(
            vec![CycloneDXVulnerabilityAffects::new(
                "a3390fca-c315-41ae-ae05-af5e7859cdee".to_string(),
            )],
            "CVE-2021-1234".to_string(),
            vec![CycloneDXVulnerabilityRating::new()
                .score(9.0 as f64)
                .severity("high".to_string())
                .vector("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:C/C:H/I:H/A:N".to_string())],
        )
        .advisories(vec![CycloneDXVulnerabilityAdvisory::new()
            .url("https://example.com/advisory/CVE-2021-1234".to_string())])
        .analysis(CycloneDXVulnerabilityAnalysis::new().state("resolved".to_string()))
        .cwes(vec![123, 345])
        .description("Sample vulnerability detected in the application.".to_string())
        .detail("Details about the vulnerability.".to_string())
        .references(vec![CycloneDXVulnerabilityReference::new()
            .id("GHSA-35m5-8cvj-8783".to_string())
            .source(
                CycloneDXVulnerabilityReferenceSource::new().url("https://example.com".to_string()),
            )])],
    )
    .version(1);
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
