// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The caller lacks edit permission on one or more feature flags that reference this saved filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SavedFilterPermissionDeniedAttributes {
    /// Feature flags preventing this update.
    #[serde(rename = "denied_flags")]
    pub denied_flags: Vec<crate::datadogV2::model::SavedFilterFlagReference>,
    /// The reason the saved filter update was rejected.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SavedFilterPermissionDeniedAttributes {
    pub fn new(
        denied_flags: Vec<crate::datadogV2::model::SavedFilterFlagReference>,
        message: String,
    ) -> SavedFilterPermissionDeniedAttributes {
        SavedFilterPermissionDeniedAttributes {
            denied_flags,
            message,
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

impl<'de> Deserialize<'de> for SavedFilterPermissionDeniedAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SavedFilterPermissionDeniedAttributesVisitor;
        impl<'a> Visitor<'a> for SavedFilterPermissionDeniedAttributesVisitor {
            type Value = SavedFilterPermissionDeniedAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut denied_flags: Option<
                    Vec<crate::datadogV2::model::SavedFilterFlagReference>,
                > = None;
                let mut message: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "denied_flags" => {
                            denied_flags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let denied_flags =
                    denied_flags.ok_or_else(|| M::Error::missing_field("denied_flags"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;

                let content = SavedFilterPermissionDeniedAttributes {
                    denied_flags,
                    message,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SavedFilterPermissionDeniedAttributesVisitor)
    }
}
