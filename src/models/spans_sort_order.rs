/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansSortOrder : The order to use, ascending or descending.

/// The order to use, ascending or descending.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpansSortOrder {
    #[serde(rename = "asc")]
    ASCENDING,
    #[serde(rename = "desc")]
    DESCENDING,

}

impl ToString for SpansSortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::ASCENDING => String::from("asc"),
            Self::DESCENDING => String::from("desc"),
        }
    }
}

impl Default for SpansSortOrder {
    fn default() -> SpansSortOrder {
        Self::ASCENDING
    }
}




