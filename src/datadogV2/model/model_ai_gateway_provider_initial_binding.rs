// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Reserved initial provider access binding. Initial provider sharing is unavailable.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayProviderInitialBinding {
    /// Access principals in type:id form.
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// Provider access relation. Editor also grants View.
    #[serde(rename = "relation")]
    pub relation: crate::datadogV2::model::AiGatewayProviderInitialBindingRelation,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayProviderInitialBinding {
    pub fn new(
        principals: Vec<String>,
        relation: crate::datadogV2::model::AiGatewayProviderInitialBindingRelation,
    ) -> AiGatewayProviderInitialBinding {
        AiGatewayProviderInitialBinding {
            principals,
            relation,
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

impl<'de> Deserialize<'de> for AiGatewayProviderInitialBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayProviderInitialBindingVisitor;
        impl<'a> Visitor<'a> for AiGatewayProviderInitialBindingVisitor {
            type Value = AiGatewayProviderInitialBinding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut principals: Option<Vec<String>> = None;
                let mut relation: Option<
                    crate::datadogV2::model::AiGatewayProviderInitialBindingRelation,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "principals" => {
                            principals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relation" => {
                            relation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _relation) = relation {
                                match _relation {
                                    crate::datadogV2::model::AiGatewayProviderInitialBindingRelation::UnparsedObject(_relation) => {
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
                let principals = principals.ok_or_else(|| M::Error::missing_field("principals"))?;
                let relation = relation.ok_or_else(|| M::Error::missing_field("relation"))?;

                let content = AiGatewayProviderInitialBinding {
                    principals,
                    relation,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayProviderInitialBindingVisitor)
    }
}
