// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsExclusionFilter {
    /// Default query is `*`, meaning all logs flowing in the index would be excluded.
Scope down exclusion filter to only a subset of logs with a log query.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// Sample rate to apply to logs going through this exclusion filter,
a value of 1.0 excludes all logs matching the query.
    #[serde(rename = "sample_rate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: f64,
}

