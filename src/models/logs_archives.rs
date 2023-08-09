/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsArchives : The available archives.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LogsArchives {
    /// A list of archives.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::LogsArchiveDefinition>>,
}

impl LogsArchives {
    /// The available archives.
    pub fn new() -> LogsArchives {
        LogsArchives {
            data: None,
        }
    }
}


