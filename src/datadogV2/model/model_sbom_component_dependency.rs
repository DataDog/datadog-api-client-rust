// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The dependencies of a component of the SBOM.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SBOMComponentDependency {
    /// The components that are dependencies of the ref component.
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<Vec<String>>,
    /// The identifier for the related component.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SBOMComponentDependency {
    pub fn new() -> SBOMComponentDependency {
        SBOMComponentDependency {
            depends_on: None,
            ref_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn depends_on(mut self, value: Vec<String>) -> Self {
        self.depends_on = Some(value);
        self
    }

    pub fn ref_(mut self, value: String) -> Self {
        self.ref_ = Some(value);
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

impl Default for SBOMComponentDependency {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SBOMComponentDependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SBOMComponentDependencyVisitor;
        impl<'a> Visitor<'a> for SBOMComponentDependencyVisitor {
            type Value = SBOMComponentDependency;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut depends_on: Option<Vec<String>> = None;
                let mut ref_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dependsOn" => {
                            if v.is_null() {
                                continue;
                            }
                            depends_on = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ref" => {
                            if v.is_null() {
                                continue;
                            }
                            ref_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SBOMComponentDependency {
                    depends_on,
                    ref_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SBOMComponentDependencyVisitor)
    }
}
