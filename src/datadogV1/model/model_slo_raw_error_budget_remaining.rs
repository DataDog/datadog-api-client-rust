// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Error budget remaining for an SLO.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLORawErrorBudgetRemaining {
    /// Error budget remaining unit.
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /// Error budget remaining value.
    #[serde(rename = "value")]
    pub value: Option<f64>,
}

impl SLORawErrorBudgetRemaining {
    pub fn new() -> SLORawErrorBudgetRemaining {
        SLORawErrorBudgetRemaining {
            unit: None,
            value: None,
        }
    }

    pub fn unit(&mut self, value: String) -> &mut Self {
        self.unit = Some(value);
        self
    }

    pub fn value(&mut self, value: f64) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Default for SLORawErrorBudgetRemaining {
    fn default() -> Self {
        Self::new()
    }
}
