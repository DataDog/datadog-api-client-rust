// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API attributes for an outcome.
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
}
impl Default for OutcomesBatchResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}
