// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ActionConnectionAttributesUpdate` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionConnectionAttributesUpdate {
    /// The definition of `ActionConnectionIntegrationUpdate` object.
    #[serde(rename = "integration")]
    pub integration: Option<crate::datadogV2::model::ActionConnectionIntegrationUpdate>,
    /// Name of the connection
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionConnectionAttributesUpdate {
    pub fn new() -> ActionConnectionAttributesUpdate {
        ActionConnectionAttributesUpdate {
            integration: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn integration(
        mut self,
        value: crate::datadogV2::model::ActionConnectionIntegrationUpdate,
    ) -> Self {
        self.integration = Some(value);
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

impl Default for ActionConnectionAttributesUpdate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ActionConnectionAttributesUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionConnectionAttributesUpdateVisitor;
        impl<'a> Visitor<'a> for ActionConnectionAttributesUpdateVisitor {
            type Value = ActionConnectionAttributesUpdate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut integration: Option<
                    crate::datadogV2::model::ActionConnectionIntegrationUpdate,
                > = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "integration" => {
                            if v.is_null() {
                                continue;
                            }
                            integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration) = integration {
                                match _integration {
                                    crate::datadogV2::model::ActionConnectionIntegrationUpdate::UnparsedObject(_integration) => {
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

                let content = ActionConnectionAttributesUpdate {
                    integration,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionConnectionAttributesUpdateVisitor)
    }
}
