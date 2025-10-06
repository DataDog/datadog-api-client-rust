// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ScaRequestDataAttributesDependenciesItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesDependenciesItems {
    /// The `items` `exclusions`.
    #[serde(rename = "exclusions")]
    pub exclusions: Option<Vec<String>>,
    /// The `items` `group`.
    #[serde(rename = "group")]
    pub group: Option<String>,
    /// The `items` `is_dev`.
    #[serde(rename = "is_dev")]
    pub is_dev: Option<bool>,
    /// The `items` `is_direct`.
    #[serde(rename = "is_direct")]
    pub is_direct: Option<bool>,
    /// The `items` `language`.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// The `items` `locations`.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems>>,
    /// The `items` `name`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The `items` `package_manager`.
    #[serde(rename = "package_manager")]
    pub package_manager: Option<String>,
    /// The `items` `purl`.
    #[serde(rename = "purl")]
    pub purl: Option<String>,
    /// The `items` `reachable_symbol_properties`.
    #[serde(rename = "reachable_symbol_properties")]
    pub reachable_symbol_properties: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems>>,
    /// The `items` `version`.
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScaRequestDataAttributesDependenciesItems {
    pub fn new() -> ScaRequestDataAttributesDependenciesItems {
        ScaRequestDataAttributesDependenciesItems {
            exclusions: None,
            group: None,
            is_dev: None,
            is_direct: None,
            language: None,
            locations: None,
            name: None,
            package_manager: None,
            purl: None,
            reachable_symbol_properties: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclusions(mut self, value: Vec<String>) -> Self {
        self.exclusions = Some(value);
        self
    }

    pub fn group(mut self, value: String) -> Self {
        self.group = Some(value);
        self
    }

    pub fn is_dev(mut self, value: bool) -> Self {
        self.is_dev = Some(value);
        self
    }

    pub fn is_direct(mut self, value: bool) -> Self {
        self.is_direct = Some(value);
        self
    }

    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    pub fn locations(
        mut self,
        value: Vec<
            crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems,
        >,
    ) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn package_manager(mut self, value: String) -> Self {
        self.package_manager = Some(value);
        self
    }

    pub fn purl(mut self, value: String) -> Self {
        self.purl = Some(value);
        self
    }

    pub fn reachable_symbol_properties(
        mut self,
        value: Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems>,
    ) -> Self {
        self.reachable_symbol_properties = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
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

impl Default for ScaRequestDataAttributesDependenciesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesDependenciesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesDependenciesItemsVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesDependenciesItemsVisitor {
            type Value = ScaRequestDataAttributesDependenciesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclusions: Option<Vec<String>> = None;
                let mut group: Option<String> = None;
                let mut is_dev: Option<bool> = None;
                let mut is_direct: Option<bool> = None;
                let mut language: Option<String> = None;
                let mut locations: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems>> = None;
                let mut name: Option<String> = None;
                let mut package_manager: Option<String> = None;
                let mut purl: Option<String> = None;
                let mut reachable_symbol_properties: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems>> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclusions" => {
                            if v.is_null() {
                                continue;
                            }
                            exclusions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_dev" => {
                            if v.is_null() {
                                continue;
                            }
                            is_dev = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_direct" => {
                            if v.is_null() {
                                continue;
                            }
                            is_direct = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locations" => {
                            if v.is_null() {
                                continue;
                            }
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "package_manager" => {
                            if v.is_null() {
                                continue;
                            }
                            package_manager =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "purl" => {
                            if v.is_null() {
                                continue;
                            }
                            purl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reachable_symbol_properties" => {
                            if v.is_null() {
                                continue;
                            }
                            reachable_symbol_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ScaRequestDataAttributesDependenciesItems {
                    exclusions,
                    group,
                    is_dev,
                    is_direct,
                    language,
                    locations,
                    name,
                    package_manager,
                    purl,
                    reachable_symbol_properties,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesDependenciesItemsVisitor)
    }
}
