// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Event creation response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventCreateResponsePayload {
    /// Event object.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::EventCreateResponse>,
    /// Links to the event.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::EventCreateResponsePayloadLinks>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventCreateResponsePayload {
    pub fn new() -> EventCreateResponsePayload {
        EventCreateResponsePayload {
            data: None,
            links: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::EventCreateResponse) -> Self {
        self.data = Some(value);
        self
    }

    pub fn links(
        mut self,
        value: crate::datadogV2::model::EventCreateResponsePayloadLinks,
    ) -> Self {
        self.links = Some(value);
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

impl Default for EventCreateResponsePayload {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EventCreateResponsePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventCreateResponsePayloadVisitor;
        impl<'a> Visitor<'a> for EventCreateResponsePayloadVisitor {
            type Value = EventCreateResponsePayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::EventCreateResponse> = None;
                let mut links: Option<crate::datadogV2::model::EventCreateResponsePayloadLinks> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EventCreateResponsePayload {
                    data,
                    links,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventCreateResponsePayloadVisitor)
    }
}
