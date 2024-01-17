// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The funnel step.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunnelStep {
    /// The facet of the step.
    #[serde(rename = "facet")]
    pub facet: String,
    /// The value of the step.
    #[serde(rename = "value")]
    pub value: String,
}

impl FunnelStep {
    pub fn new(facet: String, value: String) -> FunnelStep {
        FunnelStep { facet, value }
    }
}
