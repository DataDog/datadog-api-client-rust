// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamWidgetRequest {
    /// Widget columns.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Vec<ListStreamColumn>,
    /// Updated list stream widget.
    #[serde(rename = "query")]
    pub query: ListStreamQuery,
    /// Widget response format.
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: ListStreamResponseFormat,
}

