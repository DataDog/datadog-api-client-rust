/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeseriesQuery : An individual timeseries query to one of the basic Datadog data sources.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeseriesQuery {
    #[serde(rename = "data_source")]
    pub data_source: crate::models::EventsDataSource,
    /// The variable name for use in formulas.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A classic metrics query string.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "compute")]
    pub compute: Box<crate::models::EventsCompute>,
    /// The list of facets on which to split results.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<crate::models::EventsGroupBy>>,
    /// The indexes in which to search.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<String>>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Box<crate::models::EventsSearch>>,
}

impl TimeseriesQuery {
    /// An individual timeseries query to one of the basic Datadog data sources.
    pub fn new(data_source: crate::models::EventsDataSource, query: String, compute: crate::models::EventsCompute) -> TimeseriesQuery {
        TimeseriesQuery {
            data_source,
            name: None,
            query,
            compute: Box::new(compute),
            group_by: None,
            indexes: None,
            search: None,
        }
    }
}


