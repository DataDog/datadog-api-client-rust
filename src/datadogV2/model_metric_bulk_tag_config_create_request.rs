// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigCreateRequest {
    /// Request object to bulk configure tags for metrics matching the given prefix.
    #[serde(rename = "data")]
    pub data: MetricBulkTagConfigCreate,
}

