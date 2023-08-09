/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsArchiveDestinationS3 : The S3 archive destination.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsArchiveDestinationS3 {
    /// The bucket where the archive will be stored.
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "integration")]
    pub integration: Box<crate::models::LogsArchiveIntegrationS3>,
    /// The archive path.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub r#type: crate::models::LogsArchiveDestinationS3Type,
}

impl LogsArchiveDestinationS3 {
    /// The S3 archive destination.
    pub fn new(bucket: String, integration: crate::models::LogsArchiveIntegrationS3, r#type: crate::models::LogsArchiveDestinationS3Type) -> LogsArchiveDestinationS3 {
        LogsArchiveDestinationS3 {
            bucket,
            integration: Box::new(integration),
            path: None,
            r#type,
        }
    }
}


