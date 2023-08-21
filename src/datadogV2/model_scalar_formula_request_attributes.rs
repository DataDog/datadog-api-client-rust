// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Vec<QueryFormula>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: i64,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Vec<ScalarQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: i64,
}

