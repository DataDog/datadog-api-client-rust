// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service owner's Microsoft Teams.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2MSTeams {
    /// Contact value.
    #[serde(rename = "contact")]
    pub contact: String,
    /// Contact Microsoft Teams.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Contact type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ServiceDefinitionV2MSTeamsType,
}

impl ServiceDefinitionV2MSTeams {
    pub fn new(
        contact: String,
        type_: crate::datadogV2::model::ServiceDefinitionV2MSTeamsType,
    ) -> ServiceDefinitionV2MSTeams {
        ServiceDefinitionV2MSTeams {
            contact,
            name: None,
            type_,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}
