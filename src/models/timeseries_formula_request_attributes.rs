/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeseriesFormulaRequestAttributes : The object describing a timeseries formula request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeseriesFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Option<Vec<crate::models::QueryFormula>>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// A time interval in milliseconds. May be overridden by a larger interval if the query would result in too many points for the specified timeframe. Defaults to a reasonable interval for the given timeframe.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::models::TimeseriesQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
}

impl TimeseriesFormulaRequestAttributes {
    /// The object describing a timeseries formula request.
    pub fn new(from: i64, queries: Vec<crate::models::TimeseriesQuery>, to: i64) -> TimeseriesFormulaRequestAttributes {
        TimeseriesFormulaRequestAttributes {
            formulas: None,
            from,
            interval: None,
            queries,
            to,
        }
    }
}


