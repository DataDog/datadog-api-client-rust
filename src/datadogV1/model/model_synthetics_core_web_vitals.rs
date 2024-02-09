// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Core Web Vitals attached to a browser test step.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCoreWebVitals {
    /// Cumulative Layout Shift.
    #[serde(rename = "cls")]
    pub cls: Option<f64>,
    /// Largest Contentful Paint in milliseconds.
    #[serde(rename = "lcp")]
    pub lcp: Option<f64>,
    /// URL attached to the metrics.
    #[serde(rename = "url")]
    pub url: Option<String>,
}

impl SyntheticsCoreWebVitals {
    pub fn new() -> SyntheticsCoreWebVitals {
        SyntheticsCoreWebVitals {
            cls: None,
            lcp: None,
            url: None,
        }
    }

    pub fn cls(&mut self, value: f64) -> &mut Self {
        self.cls = Some(value);
        self
    }

    pub fn lcp(&mut self, value: f64) -> &mut Self {
        self.lcp = Some(value);
        self
    }

    pub fn url(&mut self, value: String) -> &mut Self {
        self.url = Some(value);
        self
    }
}

impl Default for SyntheticsCoreWebVitals {
    fn default() -> Self {
        Self::new()
    }
}
