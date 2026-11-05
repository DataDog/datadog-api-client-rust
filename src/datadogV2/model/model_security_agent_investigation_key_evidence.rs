// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A piece of evidence the verdict rests on.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentInvestigationKeyEvidence {
    /// Category of the evidence.
    #[serde(rename = "category")]
    pub category: String,
    /// What the investigation found.
    #[serde(rename = "finding")]
    pub finding: String,
    /// Names of the steps the evidence comes from.
    #[serde(rename = "source_steps")]
    pub source_steps: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentInvestigationKeyEvidence {
    pub fn new(category: String, finding: String) -> SecurityAgentInvestigationKeyEvidence {
        SecurityAgentInvestigationKeyEvidence {
            category,
            finding,
            source_steps: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn source_steps(mut self, value: Vec<String>) -> Self {
        self.source_steps = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentInvestigationKeyEvidence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentInvestigationKeyEvidenceVisitor;
        impl<'a> Visitor<'a> for SecurityAgentInvestigationKeyEvidenceVisitor {
            type Value = SecurityAgentInvestigationKeyEvidence;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut finding: Option<String> = None;
                let mut source_steps: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "finding" => {
                            finding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_steps" => {
                            if v.is_null() {
                                continue;
                            }
                            source_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let finding = finding.ok_or_else(|| M::Error::missing_field("finding"))?;

                let content = SecurityAgentInvestigationKeyEvidence {
                    category,
                    finding,
                    source_steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentInvestigationKeyEvidenceVisitor)
    }
}
