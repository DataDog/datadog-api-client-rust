// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a Google Chat target audience.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatTargetAudienceUpdateRequestAttributes {
    /// The audience ID.
    #[serde(rename = "audience_id")]
    pub audience_id: Option<String>,
    /// The audience name.
    #[serde(rename = "audience_name")]
    pub audience_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatTargetAudienceUpdateRequestAttributes {
    pub fn new() -> GoogleChatTargetAudienceUpdateRequestAttributes {
        GoogleChatTargetAudienceUpdateRequestAttributes {
            audience_id: None,
            audience_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn audience_id(mut self, value: String) -> Self {
        self.audience_id = Some(value);
        self
    }

    pub fn audience_name(mut self, value: String) -> Self {
        self.audience_name = Some(value);
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

impl Default for GoogleChatTargetAudienceUpdateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GoogleChatTargetAudienceUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatTargetAudienceUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for GoogleChatTargetAudienceUpdateRequestAttributesVisitor {
            type Value = GoogleChatTargetAudienceUpdateRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut audience_id: Option<String> = None;
                let mut audience_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "audience_id" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audience_name" => {
                            if v.is_null() {
                                continue;
                            }
                            audience_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GoogleChatTargetAudienceUpdateRequestAttributes {
                    audience_id,
                    audience_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatTargetAudienceUpdateRequestAttributesVisitor)
    }
}
