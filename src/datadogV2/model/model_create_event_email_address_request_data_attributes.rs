// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateEventEmailAddressRequestDataAttributes {
    #[serde(rename = "alert_type")]
    pub alert_type: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "format")]
    pub format: String,
    #[serde(rename = "notify_handles")]
    pub notify_handles: Vec<String>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateEventEmailAddressRequestDataAttributes {
    pub fn new(
        format: String,
        notify_handles: Vec<String>,
        tags: Vec<String>,
    ) -> CreateEventEmailAddressRequestDataAttributes {
        CreateEventEmailAddressRequestDataAttributes {
            alert_type: None,
            description: None,
            format,
            notify_handles,
            tags,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn alert_type(mut self, value: String) -> Self {
        self.alert_type = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for CreateEventEmailAddressRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateEventEmailAddressRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CreateEventEmailAddressRequestDataAttributesVisitor {
            type Value = CreateEventEmailAddressRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut alert_type: Option<String> = None;
                let mut description: Option<String> = None;
                let mut format: Option<String> = None;
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
                            if v.is_null() {
                                continue;
                            }
                            alert_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let format = format.ok_or_else(|| M::Error::missing_field("format"))?;
                let notify_handles =
                    notify_handles.ok_or_else(|| M::Error::missing_field("notify_handles"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = CreateEventEmailAddressRequestDataAttributes {
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

        deserializer.deserialize_any(CreateEventEmailAddressRequestDataAttributesVisitor)
    }
}
