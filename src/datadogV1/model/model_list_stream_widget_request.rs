// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated list stream widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamWidgetRequest {
    /// Widget columns.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV1::model::ListStreamColumn>,
    /// Updated list stream widget.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::ListStreamQuery,
    /// Widget response format.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::ListStreamResponseFormat,
}

impl ListStreamWidgetRequest {
    pub fn new(
        columns: Vec<crate::datadogV1::model::ListStreamColumn>,
        query: crate::datadogV1::model::ListStreamQuery,
        response_format: crate::datadogV1::model::ListStreamResponseFormat,
    ) -> ListStreamWidgetRequest {
        ListStreamWidgetRequest {
            columns,
            query,
            response_format,
        }
    }
}
