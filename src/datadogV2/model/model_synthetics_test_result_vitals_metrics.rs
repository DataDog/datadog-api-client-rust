// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Web vitals metrics captured during a browser test step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestResultVitalsMetrics {
    /// Cumulative Layout Shift score.
    #[serde(rename = "cls")]
    pub cls: Option<f64>,
    /// First Contentful Paint in milliseconds.
    #[serde(rename = "fcp")]
    pub fcp: Option<f64>,
    /// Interaction to Next Paint in milliseconds.
    #[serde(rename = "inp")]
    pub inp: Option<f64>,
    /// Largest Contentful Paint in milliseconds.
    #[serde(rename = "lcp")]
    pub lcp: Option<f64>,
    /// Time To First Byte in milliseconds.
    #[serde(rename = "ttfb")]
    pub ttfb: Option<f64>,
    /// URL that produced the metrics.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestResultVitalsMetrics {
    pub fn new() -> SyntheticsTestResultVitalsMetrics {
        SyntheticsTestResultVitalsMetrics {
            cls: None,
            fcp: None,
            inp: None,
            lcp: None,
            ttfb: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cls(mut self, value: f64) -> Self {
        self.cls = Some(value);
        self
    }

    pub fn fcp(mut self, value: f64) -> Self {
        self.fcp = Some(value);
        self
    }

    pub fn inp(mut self, value: f64) -> Self {
        self.inp = Some(value);
        self
    }

    pub fn lcp(mut self, value: f64) -> Self {
        self.lcp = Some(value);
        self
    }

    pub fn ttfb(mut self, value: f64) -> Self {
        self.ttfb = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SyntheticsTestResultVitalsMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestResultVitalsMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestResultVitalsMetricsVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestResultVitalsMetricsVisitor {
            type Value = SyntheticsTestResultVitalsMetrics;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cls: Option<f64> = None;
                let mut fcp: Option<f64> = None;
                let mut inp: Option<f64> = None;
                let mut lcp: Option<f64> = None;
                let mut ttfb: Option<f64> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cls" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            cls = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "fcp" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            fcp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inp" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            inp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lcp" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            lcp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ttfb" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            ttfb = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestResultVitalsMetrics {
                    cls,
                    fcp,
                    inp,
                    lcp,
                    ttfb,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestResultVitalsMetricsVisitor)
    }
}
