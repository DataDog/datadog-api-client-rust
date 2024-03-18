// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team permission setting update attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamPermissionSettingUpdateAttributes {
    /// What type of user is allowed to perform the specified action
    #[serde(rename = "value")]
    pub value: Option<crate::datadogV2::model::TeamPermissionSettingValue>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamPermissionSettingUpdateAttributes {
    pub fn new() -> TeamPermissionSettingUpdateAttributes {
        TeamPermissionSettingUpdateAttributes {
            value: None,
            _unparsed: false,
        }
    }

    pub fn value(mut self, value: crate::datadogV2::model::TeamPermissionSettingValue) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for TeamPermissionSettingUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamPermissionSettingUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamPermissionSettingUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for TeamPermissionSettingUpdateAttributesVisitor {
            type Value = TeamPermissionSettingUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut value: Option<crate::datadogV2::model::TeamPermissionSettingValue> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value) = value {
                                match _value {
                                    crate::datadogV2::model::TeamPermissionSettingValue::UnparsedObject(_value) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = TeamPermissionSettingUpdateAttributes { value, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamPermissionSettingUpdateAttributesVisitor)
    }
}
