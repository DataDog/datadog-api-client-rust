// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard outcome for a specific rule, for a given service within a batched update.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesBatchRequestItem {
    /// Any remarks regarding the scorecard rule's evaluation, and supports HTML hyperlinks.
    #[serde(rename = "remarks")]
    pub remarks: Option<String>,
    /// The unique ID for a scorecard rule.
    #[serde(rename = "rule_id")]
    pub rule_id: String,
    /// The unique name for a service in the catalog.
    #[serde(rename = "service_name")]
    pub service_name: String,
    /// The state of the rule evaluation.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::State,
}

impl OutcomesBatchRequestItem {
    pub fn new(
        rule_id: String,
        service_name: String,
        state: crate::datadogV2::model::State,
    ) -> OutcomesBatchRequestItem {
        OutcomesBatchRequestItem {
            remarks: None,
            rule_id,
            service_name,
            state,
        }
    }
}
