/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AuditLogsSort : Sort parameters when querying events.

/// Sort parameters when querying events.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuditLogsSort {
    #[serde(rename = "timestamp")]
    TIMESTAMP_ASCENDING,
    #[serde(rename = "-timestamp")]
    TIMESTAMP_DESCENDING,

}

impl ToString for AuditLogsSort {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESTAMP_ASCENDING => String::from("timestamp"),
            Self::TIMESTAMP_DESCENDING => String::from("-timestamp"),
        }
    }
}

impl Default for AuditLogsSort {
    fn default() -> AuditLogsSort {
        Self::TIMESTAMP_ASCENDING
    }
}




