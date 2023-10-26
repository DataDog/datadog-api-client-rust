// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListFindingsResponse {
    /// Array of findings.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::Finding>,
    /// Metadata for pagination.
    #[serde(rename = "meta")]
    pub meta: Box<crate::datadogV2::model::ListFindingsMeta>,
}

impl ListFindingsResponse {
    /// The expected response schema when listing findings.
    pub fn new(
        data: Vec<crate::datadogV2::model::Finding>,
        meta: crate::datadogV2::model::ListFindingsMeta,
    ) -> ListFindingsResponse {
        ListFindingsResponse {
            data: data,
            meta: Box::new(meta),
        }
    }
}
