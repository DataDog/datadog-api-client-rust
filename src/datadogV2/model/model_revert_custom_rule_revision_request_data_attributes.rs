// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RevertCustomRuleRevisionRequestDataAttributes {
    /// Current revision ID
    #[serde(rename = "currentRevision")]
    pub current_revision: Option<String>,
    /// Target revision ID to revert to
    #[serde(rename = "revertToRevision")]
    pub revert_to_revision: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RevertCustomRuleRevisionRequestDataAttributes {
    pub fn new() -> RevertCustomRuleRevisionRequestDataAttributes {
        RevertCustomRuleRevisionRequestDataAttributes {
            current_revision: None,
            revert_to_revision: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn current_revision(mut self, value: String) -> Self {
        self.current_revision = Some(value);
        self
    }

    pub fn revert_to_revision(mut self, value: String) -> Self {
        self.revert_to_revision = Some(value);
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

impl Default for RevertCustomRuleRevisionRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RevertCustomRuleRevisionRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RevertCustomRuleRevisionRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for RevertCustomRuleRevisionRequestDataAttributesVisitor {
            type Value = RevertCustomRuleRevisionRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut current_revision: Option<String> = None;
                let mut revert_to_revision: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "currentRevision" => {
                            if v.is_null() {
                                continue;
                            }
                            current_revision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revertToRevision" => {
                            if v.is_null() {
                                continue;
                            }
                            revert_to_revision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RevertCustomRuleRevisionRequestDataAttributes {
                    current_revision,
                    revert_to_revision,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RevertCustomRuleRevisionRequestDataAttributesVisitor)
    }
}
