// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Synthetic location.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTriggerCITestLocation {
    /// Unique identifier of the location.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Name of the location.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl SyntheticsTriggerCITestLocation {
    pub fn new() -> SyntheticsTriggerCITestLocation {
        SyntheticsTriggerCITestLocation {
            id: None,
            name: None,
        }
    }
}
impl Default for SyntheticsTriggerCITestLocation {
    fn default() -> Self {
        Self::new()
    }
}
