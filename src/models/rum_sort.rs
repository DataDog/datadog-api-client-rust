/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// RumSort : Sort parameters when querying events.

/// Sort parameters when querying events.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RumSort {
    #[serde(rename = "timestamp")]
    TIMESTAMP_ASCENDING,
    #[serde(rename = "-timestamp")]
    TIMESTAMP_DESCENDING,

}

impl ToString for RumSort {
    fn to_string(&self) -> String {
        match self {
            Self::TIMESTAMP_ASCENDING => String::from("timestamp"),
            Self::TIMESTAMP_DESCENDING => String::from("-timestamp"),
        }
    }
}

impl Default for RumSort {
    fn default() -> RumSort {
        Self::TIMESTAMP_ASCENDING
    }
}




