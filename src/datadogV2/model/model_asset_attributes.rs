// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API attributes of the asset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssetAttributes {
    /// Asset architecture.
    #[serde(rename = "arch")]
    pub arch: Option<String>,
    /// List of environments where the asset is deployed.
    #[serde(rename = "environments")]
    pub environments: Vec<String>,
    /// Asset name.
    #[serde(rename = "name")]
    pub name: String,
    /// Asset operating system.
    #[serde(rename = "operating_system")]
    pub operating_system: Option<crate::datadogV2::model::AssetOperatingSystem>,
    /// Asset risks.
    #[serde(rename = "risks")]
    pub risks: crate::datadogV2::model::AssetRisks,
    /// List of teams that own the asset.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,
    /// The asset type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AssetType,
    /// Asset version.
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV2::model::AssetVersion>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AssetAttributes {
    pub fn new(
        environments: Vec<String>,
        name: String,
        risks: crate::datadogV2::model::AssetRisks,
        type_: crate::datadogV2::model::AssetType,
    ) -> AssetAttributes {
        AssetAttributes {
            arch: None,
            environments,
            name,
            operating_system: None,
            risks,
            teams: None,
            type_,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn arch(mut self, value: String) -> Self {
        self.arch = Some(value);
        self
    }

    pub fn operating_system(
        mut self,
        value: crate::datadogV2::model::AssetOperatingSystem,
    ) -> Self {
        self.operating_system = Some(value);
        self
    }

    pub fn teams(mut self, value: Vec<String>) -> Self {
        self.teams = Some(value);
        self
    }

    pub fn version(mut self, value: crate::datadogV2::model::AssetVersion) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for AssetAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AssetAttributesVisitor;
        impl<'a> Visitor<'a> for AssetAttributesVisitor {
            type Value = AssetAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arch: Option<String> = None;
                let mut environments: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut operating_system: Option<crate::datadogV2::model::AssetOperatingSystem> =
                    None;
                let mut risks: Option<crate::datadogV2::model::AssetRisks> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV2::model::AssetType> = None;
                let mut version: Option<crate::datadogV2::model::AssetVersion> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arch" => {
                            if v.is_null() {
                                continue;
                            }
                            arch = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environments" => {
                            environments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operating_system" => {
                            if v.is_null() {
                                continue;
                            }
                            operating_system =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risks" => {
                            risks = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            if v.is_null() {
                                continue;
                            }
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AssetType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let environments =
                    environments.ok_or_else(|| M::Error::missing_field("environments"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let risks = risks.ok_or_else(|| M::Error::missing_field("risks"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AssetAttributes {
                    arch,
                    environments,
                    name,
                    operating_system,
                    risks,
                    teams,
                    type_,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AssetAttributesVisitor)
    }
}
