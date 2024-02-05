// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API attributes for a batched set of scorecard outcomes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesBatchAttributes {
    /// Set of scorecard outcomes to update.
    #[serde(rename = "results")]
    pub results: Option<Vec<crate::datadogV2::model::OutcomesBatchRequestItem>>,
}

impl OutcomesBatchAttributes {
    pub fn new() -> OutcomesBatchAttributes {
        OutcomesBatchAttributes { results: None }
    }

    pub fn results(
        &mut self,
        value: Vec<crate::datadogV2::model::OutcomesBatchRequestItem>,
    ) -> &mut Self {
        self.results = Some(value);
        self
    }
}

impl Default for OutcomesBatchAttributes {
    fn default() -> Self {
        Self::new()
    }
}
