// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Framework Data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateCustomFrameworkRequest {
    /// Framework Description
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Framework Handle
    #[serde(rename = "handle")]
    pub handle: String,
    /// Framework Icon URL
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    /// Framework Name
    #[serde(rename = "name")]
    pub name: String,
    /// Framework Requirements
    #[serde(rename = "requirements")]
    pub requirements: Vec<crate::datadogV2::model::FrameworkRequirement>,
    /// Framework Version
    #[serde(rename = "version")]
    pub version: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateCustomFrameworkRequest {
    pub fn new(
        handle: String,
        name: String,
        requirements: Vec<crate::datadogV2::model::FrameworkRequirement>,
        version: String,
    ) -> CreateCustomFrameworkRequest {
        CreateCustomFrameworkRequest {
            description: None,
            handle,
            icon_url: None,
            name,
            requirements,
            version,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn icon_url(mut self, value: String) -> Self {
        self.icon_url = Some(value);
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

impl<'de> Deserialize<'de> for CreateCustomFrameworkRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateCustomFrameworkRequestVisitor;
        impl<'a> Visitor<'a> for CreateCustomFrameworkRequestVisitor {
            type Value = CreateCustomFrameworkRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut handle: Option<String> = None;
                let mut icon_url: Option<String> = None;
                let mut name: Option<String> = None;
                let mut requirements: Option<Vec<crate::datadogV2::model::FrameworkRequirement>> =
                    None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "handle" => {
                            handle = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon_url" => {
                            if v.is_null() {
                                continue;
                            }
                            icon_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requirements" => {
                            requirements =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let handle = handle.ok_or_else(|| M::Error::missing_field("handle"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let requirements =
                    requirements.ok_or_else(|| M::Error::missing_field("requirements"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = CreateCustomFrameworkRequest {
                    description,
                    handle,
                    icon_url,
                    name,
                    requirements,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateCustomFrameworkRequestVisitor)
    }
}
