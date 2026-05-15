// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating an app's publication protection level.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateAppProtectionLevelRequestDataAttributes {
    /// The publication protection level of the app. `approval_required` means changes must go through an approval workflow before being published.
    #[serde(rename = "protectionLevel")]
    pub protection_level: crate::datadogV2::model::AppProtectionLevel,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateAppProtectionLevelRequestDataAttributes {
    pub fn new(
        protection_level: crate::datadogV2::model::AppProtectionLevel,
    ) -> UpdateAppProtectionLevelRequestDataAttributes {
        UpdateAppProtectionLevelRequestDataAttributes {
            protection_level,
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

impl<'de> Deserialize<'de> for UpdateAppProtectionLevelRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateAppProtectionLevelRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateAppProtectionLevelRequestDataAttributesVisitor {
            type Value = UpdateAppProtectionLevelRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut protection_level: Option<crate::datadogV2::model::AppProtectionLevel> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "protectionLevel" => {
                            protection_level =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _protection_level) = protection_level {
                                match _protection_level {
                                    crate::datadogV2::model::AppProtectionLevel::UnparsedObject(
                                        _protection_level,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let protection_level =
                    protection_level.ok_or_else(|| M::Error::missing_field("protection_level"))?;

                let content = UpdateAppProtectionLevelRequestDataAttributes {
                    protection_level,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateAppProtectionLevelRequestDataAttributesVisitor)
    }
}
