// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response Object that includes your query and the list of metrics retrieved.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryResponse {
    /// Message indicating the errors if status is not `ok`.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Start of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "from_date")]
    pub from_date: Option<i64>,
    /// List of tag keys on which to group.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<String>>,
    /// Message indicating `success` if status is `ok`.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Query string
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Type of response.
    #[serde(rename = "res_type")]
    pub res_type: Option<String>,
    /// List of timeseries queried.
    #[serde(rename = "series")]
    pub series: Option<Vec<crate::datadogV1::model::MetricsQueryMetadata>>,
    /// Status of the query.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// End of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "to_date")]
    pub to_date: Option<i64>,
}

impl MetricsQueryResponse {
    pub fn new() -> MetricsQueryResponse {
        MetricsQueryResponse {
            error: None,
            from_date: None,
            group_by: None,
            message: None,
            query: None,
            res_type: None,
            series: None,
            status: None,
            to_date: None,
        }
    }

    pub fn error(&mut self, value: String) -> &mut Self {
        self.error = Some(value);
        self
    }

    pub fn from_date(&mut self, value: i64) -> &mut Self {
        self.from_date = Some(value);
        self
    }

    pub fn group_by(&mut self, value: Vec<String>) -> &mut Self {
        self.group_by = Some(value);
        self
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn query(&mut self, value: String) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn res_type(&mut self, value: String) -> &mut Self {
        self.res_type = Some(value);
        self
    }

    pub fn series(
        &mut self,
        value: Vec<crate::datadogV1::model::MetricsQueryMetadata>,
    ) -> &mut Self {
        self.series = Some(value);
        self
    }

    pub fn status(&mut self, value: String) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn to_date(&mut self, value: i64) -> &mut Self {
        self.to_date = Some(value);
        self
    }
}

impl Default for MetricsQueryResponse {
    fn default() -> Self {
        Self::new()
    }
}
