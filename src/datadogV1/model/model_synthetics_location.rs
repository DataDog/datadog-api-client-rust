// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Synthetic location that can be used when creating or editing a
/// test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsLocation {
    /// Unique identifier of the location.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of the location.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl SyntheticsLocation {
    pub fn new() -> SyntheticsLocation {
        SyntheticsLocation {
            id: None,
            name: None,
        }
    }
}