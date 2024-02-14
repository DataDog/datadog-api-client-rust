// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API attributes for an outcome.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesBatchResponseAttributes {
    /// Creation time of the rule outcome.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Time of last rule outcome modification.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<String>,
    /// Any remarks regarding the scorecard rule's evaluation, and supports HTML hyperlinks.
    #[serde(rename = "remarks")]
    pub remarks: Option<String>,
    /// The unique name for a service in the catalog.
    #[serde(rename = "service_name")]
    pub service_name: Option<String>,
    /// The state of the rule evaluation.
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::State>,
}

impl OutcomesBatchResponseAttributes {
    pub fn new() -> OutcomesBatchResponseAttributes {
        OutcomesBatchResponseAttributes {
            created_at: None,
            modified_at: None,
            remarks: None,
            service_name: None,
            state: None,
        }
    }

    pub fn created_at(&mut self, value: String) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn modified_at(&mut self, value: String) -> &mut Self {
        self.modified_at = Some(value);
        self
    }

    pub fn remarks(&mut self, value: String) -> &mut Self {
        self.remarks = Some(value);
        self
    }

    pub fn service_name(&mut self, value: String) -> &mut Self {
        self.service_name = Some(value);
        self
    }

    pub fn state(&mut self, value: crate::datadogV2::model::State) -> &mut Self {
        self.state = Some(value);
        self
    }
}

impl Default for OutcomesBatchResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
