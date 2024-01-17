// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard outcomes batch request data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesBatchRequestData {
    /// The JSON:API attributes for a batched set of scorecard outcomes.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::OutcomesBatchAttributes>>,
    /// The JSON:API type for scorecard outcomes.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::OutcomesBatchType>,
}

impl OutcomesBatchRequestData {
    pub fn new() -> OutcomesBatchRequestData {
        OutcomesBatchRequestData {
            attributes: None,
            type_: None,
        }
    }
}
