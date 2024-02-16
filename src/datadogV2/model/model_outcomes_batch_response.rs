// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard outcomes batch response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesBatchResponse {
    /// List of rule outcomes which were affected during the bulk operation.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::OutcomesResponseDataItem>,
    /// Metadata pertaining to the bulk operation.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::OutcomesBatchResponseMeta,
}

impl OutcomesBatchResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::OutcomesResponseDataItem>,
        meta: crate::datadogV2::model::OutcomesBatchResponseMeta,
    ) -> OutcomesBatchResponse {
        OutcomesBatchResponse { data, meta }
    }
}
