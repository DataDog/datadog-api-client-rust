// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMAnalyticsAggregateResponse {
    /// The query results.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: RUMAggregationBucketsResponse,
    /// Links attributes.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: RUMResponseLinks,
    /// The metadata associated with a request.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: RUMResponseMetadata,
}

