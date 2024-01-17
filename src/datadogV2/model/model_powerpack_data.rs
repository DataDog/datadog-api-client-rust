// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack data object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackData {
    /// Powerpack attribute object.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::PowerpackAttributes>>,
    /// ID of the powerpack.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Powerpack relationship object.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::PowerpackRelationships>>,
    /// Type of widget, must be powerpack.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl PowerpackData {
    pub fn new() -> PowerpackData {
        PowerpackData {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
