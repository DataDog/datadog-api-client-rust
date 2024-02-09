// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing a deleted Synthetic test ID with the associated
/// deletion timestamp.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDeletedTest {
    /// Deletion timestamp of the Synthetic test ID.
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<String>,
    /// The Synthetic test ID deleted.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl SyntheticsDeletedTest {
    pub fn new() -> SyntheticsDeletedTest {
        SyntheticsDeletedTest {
            deleted_at: None,
            public_id: None,
        }
    }

    pub fn deleted_at(&mut self, value: String) -> &mut Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn public_id(&mut self, value: String) -> &mut Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for SyntheticsDeletedTest {
    fn default() -> Self {
        Self::new()
    }
}
