// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFormula {
    /// Formula string, referencing one or more queries with their name property.
    #[serde(rename = "formula", skip_serializing_if = "Option::is_none")]
    pub formula: String,
    /// Message for specifying limits to the number of values returned by a query.
This limit is only for scalar queries and has no effect on timeseries queries.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: FormulaLimit,
}

