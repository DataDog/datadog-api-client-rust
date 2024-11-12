// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Objects describing the binding used for a mobile test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestRestrictionPolicyBinding {
    /// List of principals for a mobile test binding.
    #[serde(rename = "principals")]
    pub principals: Option<Vec<String>>,
    /// The type of relation for the binding.
    #[serde(rename = "relation")]
    pub relation: Option<crate::datadogV1::model::SyntheticsTestRestrictionPolicyBindingRelation>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestRestrictionPolicyBinding {
    pub fn new() -> SyntheticsTestRestrictionPolicyBinding {
        SyntheticsTestRestrictionPolicyBinding {
            principals: None,
            relation: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn principals(mut self, value: Vec<String>) -> Self {
        self.principals = Some(value);
        self
    }

    pub fn relation(
        mut self,
        value: crate::datadogV1::model::SyntheticsTestRestrictionPolicyBindingRelation,
    ) -> Self {
        self.relation = Some(value);
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

impl Default for SyntheticsTestRestrictionPolicyBinding {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestRestrictionPolicyBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestRestrictionPolicyBindingVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestRestrictionPolicyBindingVisitor {
            type Value = SyntheticsTestRestrictionPolicyBinding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut principals: Option<Vec<String>> = None;
                let mut relation: Option<
                    crate::datadogV1::model::SyntheticsTestRestrictionPolicyBindingRelation,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "principals" => {
                            if v.is_null() {
                                continue;
                            }
                            principals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relation" => {
                            if v.is_null() {
                                continue;
                            }
                            relation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _relation) = relation {
                                match _relation {
                                    crate::datadogV1::model::SyntheticsTestRestrictionPolicyBindingRelation::UnparsedObject(_relation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestRestrictionPolicyBinding {
                    principals,
                    relation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestRestrictionPolicyBindingVisitor)
    }
}
