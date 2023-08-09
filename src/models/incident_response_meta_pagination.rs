/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IncidentResponseMetaPagination : Pagination properties.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncidentResponseMetaPagination {
    /// The index of the first element in the next page of results. Equal to page size added to the current offset.
    #[serde(rename = "next_offset", skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<i64>,
    /// The index of the first element in the results.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum size of pages to return.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl IncidentResponseMetaPagination {
    /// Pagination properties.
    pub fn new() -> IncidentResponseMetaPagination {
        IncidentResponseMetaPagination {
            next_offset: None,
            offset: None,
            size: None,
        }
    }
}


