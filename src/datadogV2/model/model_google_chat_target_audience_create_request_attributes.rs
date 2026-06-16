// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a Google Chat target audience.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatTargetAudienceCreateRequestAttributes {
    /// The audience ID.
    #[serde(rename = "audience_id")]
    pub audience_id: String,
    /// The audience name.
    #[serde(rename = "audience_name")]
    pub audience_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatTargetAudienceCreateRequestAttributes {
    pub fn new(
        audience_id: String,
        audience_name: String,
    ) -> GoogleChatTargetAudienceCreateRequestAttributes {
        GoogleChatTargetAudienceCreateRequestAttributes {
            audience_id,
            audience_name,
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

impl<'de> Deserialize<'de> for GoogleChatTargetAudienceCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatTargetAudienceCreateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for GoogleChatTargetAudienceCreateRequestAttributesVisitor {
            type Value = GoogleChatTargetAudienceCreateRequestAttributes;

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
                            audience_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "audience_name" => {
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
                let audience_id =
                    audience_id.ok_or_else(|| M::Error::missing_field("audience_id"))?;
                let audience_name =
                    audience_name.ok_or_else(|| M::Error::missing_field("audience_name"))?;

                let content = GoogleChatTargetAudienceCreateRequestAttributes {
                    audience_id,
                    audience_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatTargetAudienceCreateRequestAttributesVisitor)
    }
}
