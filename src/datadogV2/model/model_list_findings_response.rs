// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The expected response schema when listing findings.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFindingsResponse {
    /// Array of findings.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::Finding>,
    /// Metadata for pagination.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::ListFindingsMeta,
}

impl ListFindingsResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::Finding>,
        meta: crate::datadogV2::model::ListFindingsMeta,
    ) -> ListFindingsResponse {
        ListFindingsResponse { data, meta }
    }
}
