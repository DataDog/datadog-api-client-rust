// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A recommendation-oriented summary of a candidate remediation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationProposedFix {
    /// Explanation of the proposed change and why it resolves the root cause. Treat as user-provided content and escape before display.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the proposed fix can be reversed after execution.
    #[serde(rename = "reversible")]
    pub reversible: Option<bool>,
    /// Estimated risk of a remediation step or proposed fix.
    #[serde(rename = "risk")]
    pub risk: Option<crate::datadogV2::model::RemediationRiskLevel>,
    /// Short title for the proposed fix.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationProposedFix {
    pub fn new() -> RemediationProposedFix {
        RemediationProposedFix {
            description: None,
            reversible: None,
            risk: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn reversible(mut self, value: bool) -> Self {
        self.reversible = Some(value);
        self
    }

    pub fn risk(mut self, value: crate::datadogV2::model::RemediationRiskLevel) -> Self {
        self.risk = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for RemediationProposedFix {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationProposedFix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationProposedFixVisitor;
        impl<'a> Visitor<'a> for RemediationProposedFixVisitor {
            type Value = RemediationProposedFix;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut reversible: Option<bool> = None;
                let mut risk: Option<crate::datadogV2::model::RemediationRiskLevel> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reversible" => {
                            if v.is_null() {
                                continue;
                            }
                            reversible = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risk" => {
                            if v.is_null() {
                                continue;
                            }
                            risk = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _risk) = risk {
                                match _risk {
                                    crate::datadogV2::model::RemediationRiskLevel::UnparsedObject(_risk) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationProposedFix {
                    description,
                    reversible,
                    risk,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationProposedFixVisitor)
    }
}
