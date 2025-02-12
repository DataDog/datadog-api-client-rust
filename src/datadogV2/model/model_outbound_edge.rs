// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `OutboundEdge` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutboundEdge {
    /// The `OutboundEdge` `branchName`.
    #[serde(rename = "branchName")]
    pub branch_name: String,
    /// The `OutboundEdge` `nextStepName`.
    #[serde(rename = "nextStepName")]
    pub next_step_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutboundEdge {
    pub fn new(branch_name: String, next_step_name: String) -> OutboundEdge {
        OutboundEdge {
            branch_name,
            next_step_name,
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

impl<'de> Deserialize<'de> for OutboundEdge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutboundEdgeVisitor;
        impl<'a> Visitor<'a> for OutboundEdgeVisitor {
            type Value = OutboundEdge;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut branch_name: Option<String> = None;
                let mut next_step_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "branchName" => {
                            branch_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nextStepName" => {
                            next_step_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let branch_name =
                    branch_name.ok_or_else(|| M::Error::missing_field("branch_name"))?;
                let next_step_name =
                    next_step_name.ok_or_else(|| M::Error::missing_field("next_step_name"))?;

                let content = OutboundEdge {
                    branch_name,
                    next_step_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutboundEdgeVisitor)
    }
}
