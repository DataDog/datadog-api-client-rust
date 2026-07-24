// Create postmortem template returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::ConfluencePostmortemSettings;
use datadog_api_client::datadogV2::model::GoogleDocsPostmortemSettings;
use datadog_api_client::datadogV2::model::PostmortemTemplateAttributesRequest;
use datadog_api_client::datadogV2::model::PostmortemTemplateCreateRelationships;
use datadog_api_client::datadogV2::model::PostmortemTemplateDataRequest;
use datadog_api_client::datadogV2::model::PostmortemTemplateIncidentTypeRelationship;
use datadog_api_client::datadogV2::model::PostmortemTemplateIncidentTypeRelationshipData;
use datadog_api_client::datadogV2::model::PostmortemTemplateLocation;
use datadog_api_client::datadogV2::model::PostmortemTemplateRequest;
use datadog_api_client::datadogV2::model::PostmortemTemplateType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = PostmortemTemplateRequest::new(
        PostmortemTemplateDataRequest::new(
            PostmortemTemplateAttributesRequest::new("Standard Postmortem Template".to_string())
                .confluence_postmortem_settings(
                    ConfluencePostmortemSettings::new("123456".to_string(), "789012".to_string())
                        .parent_id(Some("345678".to_string())),
                )
                .content(
                    r#"# Overview

# What Happened

# Timeline

# Action Items"#
                        .to_string(),
                )
                .google_docs_postmortem_settings(GoogleDocsPostmortemSettings::new(
                    "123456".to_string(),
                    "789012".to_string(),
                ))
                .is_default(Some(
                    DateTime::parse_from_rfc3339("2024-01-01T00:00:00+00:00")
                        .expect("Failed to parse datetime")
                        .with_timezone(&Utc),
                ))
                .location(PostmortemTemplateLocation::DATADOG_NOTEBOOKS),
            PostmortemTemplateType::POSTMORTEM_TEMPLATES,
        )
        .id("00000000-0000-0000-0000-000000000000".to_string())
        .relationships(PostmortemTemplateCreateRelationships::new().incident_type(
            PostmortemTemplateIncidentTypeRelationship::new(
                PostmortemTemplateIncidentTypeRelationshipData::new(
                    Uuid::parse_str("00000000-0000-0000-0000-000000000009").expect("invalid UUID"),
                    "incident_types".to_string(),
                ),
            ),
        )),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentPostmortemTemplate", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api.create_incident_postmortem_template(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
