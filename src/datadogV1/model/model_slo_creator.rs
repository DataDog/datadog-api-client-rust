// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The creator of the SLO
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOCreator {
    /// Email of the creator.
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// User ID of the creator.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Name of the creator.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
}

impl SLOCreator {
    pub fn new() -> SLOCreator {
        SLOCreator {
            email: None,
            id: None,
            name: None,
        }
    }
}
impl Default for SLOCreator {
    fn default() -> Self {
        Self::new()
    }
}
