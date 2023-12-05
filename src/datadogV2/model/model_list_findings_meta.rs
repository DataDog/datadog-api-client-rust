// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata for pagination.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ListFindingsMeta {
    /// Pagination and findings count information.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::ListFindingsPage>>,
    /// The point in time corresponding to the listed findings.
    #[serde(rename = "snapshot_timestamp")]
    pub snapshot_timestamp: Option<i64>,
}

impl ListFindingsMeta {
    pub fn new() -> ListFindingsMeta {
        ListFindingsMeta {
            page: None,
            snapshot_timestamp: None,
        }
    }
}