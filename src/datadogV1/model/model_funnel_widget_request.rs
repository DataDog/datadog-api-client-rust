// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated funnel widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunnelWidgetRequest {
    /// Updated funnel widget.
    #[serde(rename = "query")]
    pub query: crate::datadogV1::model::FunnelQuery,
    /// Widget request type.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::FunnelRequestType,
}

impl FunnelWidgetRequest {
    pub fn new(
        query: crate::datadogV1::model::FunnelQuery,
        request_type: crate::datadogV1::model::FunnelRequestType,
    ) -> FunnelWidgetRequest {
        FunnelWidgetRequest {
            query,
            request_type,
        }
    }
}
