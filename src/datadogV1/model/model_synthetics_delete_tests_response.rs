// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response object for deleting Synthetic tests.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDeleteTestsResponse {
    /// Array of objects containing a deleted Synthetic test ID with
    /// the associated deletion timestamp.
    #[serde(rename = "deleted_tests")]
    pub deleted_tests: Option<Vec<crate::datadogV1::model::SyntheticsDeletedTest>>,
}

impl SyntheticsDeleteTestsResponse {
    pub fn new() -> SyntheticsDeleteTestsResponse {
        SyntheticsDeleteTestsResponse {
            deleted_tests: None,
        }
    }

    pub fn with_deleted_tests(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsDeletedTest>,
    ) -> &mut Self {
        self.deleted_tests = Some(value);
        self
    }
}
impl Default for SyntheticsDeleteTestsResponse {
    fn default() -> Self {
        Self::new()
    }
}
