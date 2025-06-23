// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON object of attributes from your events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventCreateResponseAttributesAttributes {
    /// JSON object of event system attributes.
    #[serde(rename = "evt")]
    pub evt: Option<crate::datadogV2::model::EventCreateResponseAttributesAttributesEvt>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventCreateResponseAttributesAttributes {
    pub fn new() -> EventCreateResponseAttributesAttributes {
        EventCreateResponseAttributesAttributes {
            evt: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn evt(
        mut self,
        value: crate::datadogV2::model::EventCreateResponseAttributesAttributesEvt,
    ) -> Self {
        self.evt = Some(value);
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

impl Default for EventCreateResponseAttributesAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventCreateResponseAttributesAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventCreateResponseAttributesAttributesVisitor;
        impl<'a> Visitor<'a> for EventCreateResponseAttributesAttributesVisitor {
            type Value = EventCreateResponseAttributesAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut evt: Option<
                    crate::datadogV2::model::EventCreateResponseAttributesAttributesEvt,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "evt" => {
                            if v.is_null() {
                                continue;
                            }
                            evt = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EventCreateResponseAttributesAttributes {
                    evt,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventCreateResponseAttributesAttributesVisitor)
    }
}
