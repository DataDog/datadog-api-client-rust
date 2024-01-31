// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Organization object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    /// Attributes of the organization.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::OrganizationAttributes>,
    /// ID of the organization.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Organizations resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OrganizationsType,
}

impl Organization {
    pub fn new(type_: crate::datadogV2::model::OrganizationsType) -> Organization {
        Organization {
            attributes: None,
            id: None,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::OrganizationAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }
}
