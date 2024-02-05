// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An array of service level objective objects.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOBulkDeleteResponseData {
    /// An array of service level objective object IDs that indicates
    /// which objects that were completely deleted.
    #[serde(rename = "deleted")]
    pub deleted: Option<Vec<String>>,
    /// An array of service level objective object IDs that indicates
    /// which objects that were modified (objects for which at least one
    /// threshold was deleted, but that were not completely deleted).
    #[serde(rename = "updated")]
    pub updated: Option<Vec<String>>,
}

impl SLOBulkDeleteResponseData {
    pub fn new() -> SLOBulkDeleteResponseData {
        SLOBulkDeleteResponseData {
            deleted: None,
            updated: None,
        }
    }

    pub fn deleted(&mut self, value: Vec<String>) -> &mut Self {
        self.deleted = Some(value);
        self
    }

    pub fn updated(&mut self, value: Vec<String>) -> &mut Self {
        self.updated = Some(value);
        self
    }
}

impl Default for SLOBulkDeleteResponseData {
    fn default() -> Self {
        Self::new()
    }
}
