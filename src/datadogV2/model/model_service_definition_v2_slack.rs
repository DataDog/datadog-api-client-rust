// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Service owner's Slack channel.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV2Slack {
    /// Slack Channel.
    #[serde(rename = "contact")]
    pub contact: String,
    /// Contact Slack.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Contact type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ServiceDefinitionV2SlackType,
}

impl ServiceDefinitionV2Slack {
    pub fn new(
        contact: String,
        type_: crate::datadogV2::model::ServiceDefinitionV2SlackType,
    ) -> ServiceDefinitionV2Slack {
        ServiceDefinitionV2Slack {
            contact,
            name: None,
            type_,
        }
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}
