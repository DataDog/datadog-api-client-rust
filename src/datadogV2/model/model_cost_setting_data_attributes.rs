// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for a cost setting.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostSettingDataAttributes {
    /// The timestamp when the setting was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The UUID of the user who created the setting.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// The setting data as a flexible key-value map.
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option")]
    pub data: Option<Option<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// A human-readable description of the setting.
    #[serde(rename = "description")]
    pub description: String,
    /// The UUID of the user who last modified the setting.
    #[serde(rename = "last_modified_by")]
    pub last_modified_by: String,
    /// The name of the setting.
    #[serde(rename = "setting_name")]
    pub setting_name: String,
    /// The timestamp when the setting was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// The version of the setting.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostSettingDataAttributes {
    pub fn new(
        created_at: String,
        created_by: String,
        description: String,
        last_modified_by: String,
        setting_name: String,
        updated_at: String,
        version: String,
    ) -> CostSettingDataAttributes {
        CostSettingDataAttributes {
            created_at,
            created_by,
            data: None,
            description,
            last_modified_by,
            setting_name,
            updated_at,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.data = Some(value);
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

impl<'de> Deserialize<'de> for CostSettingDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostSettingDataAttributesVisitor;
        impl<'a> Visitor<'a> for CostSettingDataAttributesVisitor {
            type Value = CostSettingDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut created_by: Option<String> = None;
                let mut data: Option<
                    Option<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut description: Option<String> = None;
                let mut last_modified_by: Option<String> = None;
                let mut setting_name: Option<String> = None;
                let mut updated_at: Option<String> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by" => {
                            last_modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "setting_name" => {
                            setting_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let last_modified_by =
                    last_modified_by.ok_or_else(|| M::Error::missing_field("last_modified_by"))?;
                let setting_name =
                    setting_name.ok_or_else(|| M::Error::missing_field("setting_name"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = CostSettingDataAttributes {
                    created_at,
                    created_by,
                    data,
                    description,
                    last_modified_by,
                    setting_name,
                    updated_at,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostSettingDataAttributesVisitor)
    }
}
