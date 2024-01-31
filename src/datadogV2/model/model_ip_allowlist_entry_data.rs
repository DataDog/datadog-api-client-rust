// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data of the IP allowlist entry object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPAllowlistEntryData {
    /// Attributes of the IP allowlist entry.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IPAllowlistEntryAttributes>,
    /// The unique identifier of the IP allowlist entry.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// IP allowlist Entry type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IPAllowlistEntryType,
}

impl IPAllowlistEntryData {
    pub fn new(type_: crate::datadogV2::model::IPAllowlistEntryType) -> IPAllowlistEntryData {
        IPAllowlistEntryData {
            attributes: None,
            id: None,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::IPAllowlistEntryAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }
}
