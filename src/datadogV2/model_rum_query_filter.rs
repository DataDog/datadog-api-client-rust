// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMQueryFilter {
    /// The minimum time for the requested events; supports date (in [ISO 8601](https://www.w3.org/TR/NOTE-datetime) format with full date, hours, minutes, and the `Z` UTC indicator - seconds and fractional seconds are optional), math, and regular timestamps (in milliseconds).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// The search query following the RUM search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The maximum time for the requested events; supports date (in [ISO 8601](https://www.w3.org/TR/NOTE-datetime) format with full date, hours, minutes, and the `Z` UTC indicator - seconds and fractional seconds are optional), math, and regular timestamps (in milliseconds).
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: String,
}

