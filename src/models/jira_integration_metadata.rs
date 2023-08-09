/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraIntegrationMetadata : Incident integration metadata for the Jira integration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraIntegrationMetadata {
    /// Array of Jira issues in this integration metadata.
    #[serde(rename = "issues")]
    pub issues: Vec<crate::models::JiraIntegrationMetadataIssuesItem>,
}

impl JiraIntegrationMetadata {
    /// Incident integration metadata for the Jira integration.
    pub fn new(issues: Vec<crate::models::JiraIntegrationMetadataIssuesItem>) -> JiraIntegrationMetadata {
        JiraIntegrationMetadata {
            issues,
        }
    }
}


