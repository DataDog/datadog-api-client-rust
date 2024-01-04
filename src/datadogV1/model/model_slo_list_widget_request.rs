// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated SLO List widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListWidgetRequest {
    /// Updated SLO List widget.
    #[serde(rename = "query")]
    pub query: Box<crate::datadogV1::model::SLOListWidgetQuery>,
    /// Widget request type.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::SLOListWidgetRequestType,
}

impl SLOListWidgetRequest {
    pub fn new(
        query: Box<crate::datadogV1::model::SLOListWidgetQuery>,
        request_type: crate::datadogV1::model::SLOListWidgetRequestType,
    ) -> SLOListWidgetRequest {
        SLOListWidgetRequest {
            query,
            request_type,
        }
    }
}
