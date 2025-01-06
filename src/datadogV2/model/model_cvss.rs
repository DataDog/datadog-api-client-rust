// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Vulnerability severity.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CVSS {
    /// Vulnerability severity score.
    #[serde(rename = "score")]
    pub score: f64,
    /// The vulnerability severity.
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::VulnerabilitySeverity,
    /// Vulnerability CVSS vector.
    #[serde(rename = "vector")]
    pub vector: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CVSS {
    pub fn new(
        score: f64,
        severity: crate::datadogV2::model::VulnerabilitySeverity,
        vector: String,
    ) -> CVSS {
        CVSS {
            score,
            severity,
            vector,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CVSS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CVSSVisitor;
        impl<'a> Visitor<'a> for CVSSVisitor {
            type Value = CVSS;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut score: Option<f64> = None;
                let mut severity: Option<crate::datadogV2::model::VulnerabilitySeverity> = None;
                let mut vector: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "score" => {
                            score = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::VulnerabilitySeverity::UnparsedObject(_severity) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "vector" => {
                            vector = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let score = score.ok_or_else(|| M::Error::missing_field("score"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;
                let vector = vector.ok_or_else(|| M::Error::missing_field("vector"))?;

                let content = CVSS {
                    score,
                    severity,
                    vector,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CVSSVisitor)
    }
}