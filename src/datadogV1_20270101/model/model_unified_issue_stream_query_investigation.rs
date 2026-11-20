// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UnifiedIssueStreamQueryInvestigation {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "completed")]
    pub completed: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl UnifiedIssueStreamQueryInvestigation {
    pub fn new() -> UnifiedIssueStreamQueryInvestigation {
        UnifiedIssueStreamQueryInvestigation {
            completed: None,
            _unparsed: false,
        }
    }
    
    pub fn completed(mut self, value: bool) -> Self {
        self.completed = Some(value);
        self
    }
    
}


impl Default for UnifiedIssueStreamQueryInvestigation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UnifiedIssueStreamQueryInvestigation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnifiedIssueStreamQueryInvestigationVisitor;
        impl<'a> Visitor<'a> for UnifiedIssueStreamQueryInvestigationVisitor {
            type Value = UnifiedIssueStreamQueryInvestigation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut completed: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "completed" => {
                            if v.is_null() {
                                continue;
                            }
                            completed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = UnifiedIssueStreamQueryInvestigation {
                    completed,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UnifiedIssueStreamQueryInvestigationVisitor)
    }
}
