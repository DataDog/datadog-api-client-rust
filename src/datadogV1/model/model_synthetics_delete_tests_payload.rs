// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A JSON list of the ID or IDs of the Synthetic tests that you want
/// to delete.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDeleteTestsPayload {
    /// An array of Synthetic test IDs you want to delete.
    #[serde(rename = "public_ids")]
    pub public_ids: Option<Vec<String>>,
}

impl SyntheticsDeleteTestsPayload {
    pub fn new() -> SyntheticsDeleteTestsPayload {
        SyntheticsDeleteTestsPayload { public_ids: None }
    }
}
