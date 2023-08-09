/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SpansQueryFilter : The search and filter query settings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpansQueryFilter {
    /// The minimum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The search query - following the span search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The maximum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl SpansQueryFilter {
    /// The search and filter query settings.
    pub fn new() -> SpansQueryFilter {
        SpansQueryFilter {
            from: None,
            query: None,
            to: None,
        }
    }
}


