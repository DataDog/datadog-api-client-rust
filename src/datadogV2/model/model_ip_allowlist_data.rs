// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// IP allowlist data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistData {
    /// Attributes of the IP allowlist.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::IPAllowlistAttributes>>,
    /// The unique identifier of the org.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// IP allowlist type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IPAllowlistType,
}

impl IPAllowlistData {
    pub fn new(type_: crate::datadogV2::model::IPAllowlistType) -> IPAllowlistData {
        IPAllowlistData {
            attributes: None,
            id: None,
            type_,
        }
    }
}
