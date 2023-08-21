// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListWidgetRequest {
    /// Updated SLO List widget.
    #[serde(rename = "query")]
    pub query: SLOListWidgetQuery,
    /// Widget request type.
    #[serde(rename = "request_type", skip_serializing_if = "Option::is_none")]
    pub request_type: SLOListWidgetRequestType,
}

