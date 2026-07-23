// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the ownership settings response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipSettingsAttributes {
    /// Whether automatic ownership tagging is enabled.
    #[serde(rename = "auto_tag")]
    pub auto_tag: bool,
    /// The ownership confidence level.
    #[serde(rename = "confidence_level")]
    pub confidence_level: crate::datadogV2::model::OwnershipConfidenceLevel,
    /// The current version of the ownership settings.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipSettingsAttributes {
    pub fn new(
        auto_tag: bool,
        confidence_level: crate::datadogV2::model::OwnershipConfidenceLevel,
        version: i64,
    ) -> OwnershipSettingsAttributes {
        OwnershipSettingsAttributes {
            auto_tag,
            confidence_level,
            version,
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

impl<'de> Deserialize<'de> for OwnershipSettingsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipSettingsAttributesVisitor;
        impl<'a> Visitor<'a> for OwnershipSettingsAttributesVisitor {
            type Value = OwnershipSettingsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auto_tag: Option<bool> = None;
                let mut confidence_level: Option<
                    crate::datadogV2::model::OwnershipConfidenceLevel,
                > = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auto_tag" => {
                            auto_tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_level" => {
                            confidence_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _confidence_level) = confidence_level {
                                match _confidence_level {
                                    crate::datadogV2::model::OwnershipConfidenceLevel::UnparsedObject(_confidence_level) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let auto_tag = auto_tag.ok_or_else(|| M::Error::missing_field("auto_tag"))?;
                let confidence_level =
                    confidence_level.ok_or_else(|| M::Error::missing_field("confidence_level"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = OwnershipSettingsAttributes {
                    auto_tag,
                    confidence_level,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipSettingsAttributesVisitor)
    }
}
