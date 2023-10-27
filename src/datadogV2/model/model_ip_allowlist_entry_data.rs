// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistEntryData {
    /// Attributes of the IP allowlist entry.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::IPAllowlistEntryAttributes>>,
    /// The unique identifier of the IP allowlist entry.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// IP allowlist Entry type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IPAllowlistEntryType,
}

impl IPAllowlistEntryData {
    /// Data of the IP allowlist entry object.
    pub fn new(type_: crate::datadogV2::model::IPAllowlistEntryType) -> IPAllowlistEntryData {
        IPAllowlistEntryData {
            attributes: None,
            id: None,
            type_,
        }
    }
}
