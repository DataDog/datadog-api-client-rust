// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an event email address.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventEmailAddressCreateAttributes {
    /// The alert type of events generated from the email address.
    #[serde(
        rename = "alert_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub alert_type: Option<Option<crate::datadogV2::model::EventEmailAddressAlertType>>,
    /// A description of the event email address.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The format of events ingested through the email address.
    #[serde(rename = "format")]
    pub format: crate::datadogV2::model::EventEmailAddressFormat,
    /// A list of handles to notify when an email is received.
    #[serde(rename = "notify_handles")]
    pub notify_handles: Option<Vec<String>>,
    /// A list of tags to apply to events generated from the email address.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventEmailAddressCreateAttributes {
    pub fn new(
        format: crate::datadogV2::model::EventEmailAddressFormat,
    ) -> EventEmailAddressCreateAttributes {
        EventEmailAddressCreateAttributes {
            alert_type: None,
            description: None,
            format,
            notify_handles: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alert_type(
        mut self,
        value: Option<crate::datadogV2::model::EventEmailAddressAlertType>,
    ) -> Self {
        self.alert_type = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn notify_handles(mut self, value: Vec<String>) -> Self {
        self.notify_handles = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl<'de> Deserialize<'de> for EventEmailAddressCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventEmailAddressCreateAttributesVisitor;
        impl<'a> Visitor<'a> for EventEmailAddressCreateAttributesVisitor {
            type Value = EventEmailAddressCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_type: Option<
                    Option<crate::datadogV2::model::EventEmailAddressAlertType>,
                > = None;
                let mut description: Option<String> = None;
                let mut format: Option<crate::datadogV2::model::EventEmailAddressFormat> = None;
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
                                    Some(crate::datadogV2::model::EventEmailAddressAlertType::UnparsedObject(_alert_type)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "format" => {
                            format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _format) = format {
                                match _format {
                                    crate::datadogV2::model::EventEmailAddressFormat::UnparsedObject(_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "notify_handles" => {
                            if v.is_null() {
                                continue;
                            }
                            notify_handles =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let format = format.ok_or_else(|| M::Error::missing_field("format"))?;

                let content = EventEmailAddressCreateAttributes {
                    alert_type,
                    description,
                    format,
                    notify_handles,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventEmailAddressCreateAttributesVisitor)
    }
}
