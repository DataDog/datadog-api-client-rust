// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an external team permission setting default.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExternalTeamPermissionSettingDefaultsAttributes {
    /// The permission action this default applies to.
    #[serde(rename = "action")]
    pub action: String,
    /// The external integration source for the permission setting default.
    #[serde(rename = "source")]
    pub source: crate::datadogV2::model::ExternalTeamPermissionSettingDefaultsSource,
    /// The default permission value for the action.
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExternalTeamPermissionSettingDefaultsAttributes {
    pub fn new(
        action: String,
        source: crate::datadogV2::model::ExternalTeamPermissionSettingDefaultsSource,
        value: String,
    ) -> ExternalTeamPermissionSettingDefaultsAttributes {
        ExternalTeamPermissionSettingDefaultsAttributes {
            action,
            source,
            value,
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

impl<'de> Deserialize<'de> for ExternalTeamPermissionSettingDefaultsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExternalTeamPermissionSettingDefaultsAttributesVisitor;
        impl<'a> Visitor<'a> for ExternalTeamPermissionSettingDefaultsAttributesVisitor {
            type Value = ExternalTeamPermissionSettingDefaultsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut source: Option<
                    crate::datadogV2::model::ExternalTeamPermissionSettingDefaultsSource,
                > = None;
                let mut value: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _source) = source {
                                match _source {
                                    crate::datadogV2::model::ExternalTeamPermissionSettingDefaultsSource::UnparsedObject(_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = ExternalTeamPermissionSettingDefaultsAttributes {
                    action,
                    source,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExternalTeamPermissionSettingDefaultsAttributesVisitor)
    }
}
