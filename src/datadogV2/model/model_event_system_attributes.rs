// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON object of event system attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventSystemAttributes {
    /// Event category identifying the type of event.
    #[serde(rename = "category")]
    pub category: Option<crate::datadogV2::model::EventSystemAttributesCategory>,
    /// Event identifier. This field is deprecated and will be removed in a future version. Use the `uid` field instead.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Integration ID sourced from integration manifests.
    #[serde(rename = "integration_id")]
    pub integration_id: Option<crate::datadogV2::model::EventSystemAttributesIntegrationId>,
    /// The source type ID of the event.
    #[serde(rename = "source_id")]
    pub source_id: Option<i64>,
    /// A unique identifier for the event. You can use this identifier to query or reference the event.
    #[serde(rename = "uid")]
    pub uid: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventSystemAttributes {
    pub fn new() -> EventSystemAttributes {
        EventSystemAttributes {
            category: None,
            id: None,
            integration_id: None,
            source_id: None,
            uid: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn category(
        mut self,
        value: crate::datadogV2::model::EventSystemAttributesCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn integration_id(
        mut self,
        value: crate::datadogV2::model::EventSystemAttributesIntegrationId,
    ) -> Self {
        self.integration_id = Some(value);
        self
    }

    pub fn source_id(mut self, value: i64) -> Self {
        self.source_id = Some(value);
        self
    }

    pub fn uid(mut self, value: String) -> Self {
        self.uid = Some(value);
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

impl Default for EventSystemAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventSystemAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventSystemAttributesVisitor;
        impl<'a> Visitor<'a> for EventSystemAttributesVisitor {
            type Value = EventSystemAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<crate::datadogV2::model::EventSystemAttributesCategory> =
                    None;
                let mut id: Option<String> = None;
                let mut integration_id: Option<
                    crate::datadogV2::model::EventSystemAttributesIntegrationId,
                > = None;
                let mut source_id: Option<i64> = None;
                let mut uid: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            if v.is_null() {
                                continue;
                            }
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV2::model::EventSystemAttributesCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_id" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _integration_id) = integration_id {
                                match _integration_id {
                                    crate::datadogV2::model::EventSystemAttributesIntegrationId::UnparsedObject(_integration_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "source_id" => {
                            if v.is_null() {
                                continue;
                            }
                            source_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uid" => {
                            if v.is_null() {
                                continue;
                            }
                            uid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EventSystemAttributes {
                    category,
                    id,
                    integration_id,
                    source_id,
                    uid,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventSystemAttributesVisitor)
    }
}
