// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The security filter's properties.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityFilter {
    /// The object describing a security filter.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SecurityFilterAttributes>,
    /// The ID of the security filter.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of the resource. The value should always be `security_filters`.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityFilterType>,
}

impl SecurityFilter {
    pub fn new() -> SecurityFilter {
        SecurityFilter {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::SecurityFilterAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::SecurityFilterType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SecurityFilter {
    fn default() -> Self {
        Self::new()
    }
}
