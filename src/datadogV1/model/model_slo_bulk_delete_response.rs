// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The bulk partial delete service level objective object endpoint
/// response.
///
/// This endpoint operates on multiple service level objective objects, so
/// it may be partially successful. In such cases, the "data" and "error"
/// fields in this response indicate which deletions succeeded and failed.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOBulkDeleteResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SLOBulkDeleteResponseData>>,
    /// Array of errors object returned.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<crate::datadogV1::model::SLOBulkDeleteError>>,
}

impl SLOBulkDeleteResponse {
    pub fn new() -> SLOBulkDeleteResponse {
        SLOBulkDeleteResponse {
            data: None,
            errors: None,
        }
    }
}
impl Default for SLOBulkDeleteResponse {
    fn default() -> Self {
        Self::new()
    }
}
