// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an event email address.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventEmailAddressUpdateAttributes {
    /// The alert type of events generated from the email address.
    #[serde(rename = "alert_type")]
    pub alert_type: crate::datadogV2::model::EventEmailAddressAlertType,
    /// A description of the event email address.
    #[serialize_always]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// A list of handles to notify when an email is received.
    #[serde(rename = "notify_handles")]
    pub notify_handles: Vec<String>,
    /// A list of tags to apply to events generated from the email address.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventEmailAddressUpdateAttributes {
    pub fn new(
        alert_type: crate::datadogV2::model::EventEmailAddressAlertType,
        description: Option<String>,
        notify_handles: Vec<String>,
        tags: Vec<String>,
    ) -> EventEmailAddressUpdateAttributes {
        EventEmailAddressUpdateAttributes {
            alert_type,
            description,
            notify_handles,
            tags,
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

impl<'de> Deserialize<'de> for EventEmailAddressUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventEmailAddressUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for EventEmailAddressUpdateAttributesVisitor {
            type Value = EventEmailAddressUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_type: Option<crate::datadogV2::model::EventEmailAddressAlertType> =
                    None;
                let mut description: Option<Option<String>> = None;
                let mut notify_handles: Option<Vec<String>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "alert_type" => {
                            alert_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _alert_type) = alert_type {
                                match _alert_type {
                                    crate::datadogV2::model::EventEmailAddressAlertType::UnparsedObject(_alert_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notify_handles" => {
                            notify_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let alert_type = alert_type.ok_or_else(|| M::Error::missing_field("alert_type"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let notify_handles =
                    notify_handles.ok_or_else(|| M::Error::missing_field("notify_handles"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = EventEmailAddressUpdateAttributes {
                    alert_type,
                    description,
                    notify_handles,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventEmailAddressUpdateAttributesVisitor)
    }
}
