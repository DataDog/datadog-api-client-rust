// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Vec<QueryFormula>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: i64,
    /// A time interval in milliseconds.
May be overridden by a larger interval if the query would result in
too many points for the specified timeframe.
Defaults to a reasonable interval for the given timeframe.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: i64,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<TimeseriesQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: i64,
}

