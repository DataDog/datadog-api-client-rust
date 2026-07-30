// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Updatable attributes of an Elastic Cloud integration account. Every field is optional; only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudIntegrationAccountUpdateAttributes {
    /// Partial Elastic Cloud interface for updates. Exactly one interface variant is set, selected by its `type`.
    #[serde(rename = "interface")]
    pub interface: Option<crate::datadogV2::model::ElasticCloudInterfaceUpdate>,
    /// Human-readable name of the account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudIntegrationAccountUpdateAttributes {
    pub fn new() -> ElasticCloudIntegrationAccountUpdateAttributes {
        ElasticCloudIntegrationAccountUpdateAttributes {
            interface: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn interface(
        mut self,
        value: crate::datadogV2::model::ElasticCloudInterfaceUpdate,
    ) -> Self {
        self.interface = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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

impl Default for ElasticCloudIntegrationAccountUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ElasticCloudIntegrationAccountUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudIntegrationAccountUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for ElasticCloudIntegrationAccountUpdateAttributesVisitor {
            type Value = ElasticCloudIntegrationAccountUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut interface: Option<crate::datadogV2::model::ElasticCloudInterfaceUpdate> =
                    None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "interface" => {
                            if v.is_null() {
                                continue;
                            }
                            interface = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _interface) = interface {
                                match _interface {
                                    crate::datadogV2::model::ElasticCloudInterfaceUpdate::UnparsedObject(_interface) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ElasticCloudIntegrationAccountUpdateAttributes {
                    interface,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ElasticCloudIntegrationAccountUpdateAttributesVisitor)
    }
}
