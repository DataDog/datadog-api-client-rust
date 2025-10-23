// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ScaRequestDataAttributesDependenciesItemsLocationsItems` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesDependenciesItemsLocationsItems {
    /// A position in a file
    #[serde(rename = "block")]
    pub block: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition>,
    /// A position in a file
    #[serde(rename = "name")]
    pub name: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition>,
    /// A position in a file
    #[serde(rename = "namespace")]
    pub namespace: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition>,
    /// A position in a file
    #[serde(rename = "version")]
    pub version: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScaRequestDataAttributesDependenciesItemsLocationsItems {
    pub fn new() -> ScaRequestDataAttributesDependenciesItemsLocationsItems {
        ScaRequestDataAttributesDependenciesItemsLocationsItems {
            block: None,
            name: None,
            namespace: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn block(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition,
    ) -> Self {
        self.block = Some(value);
        self
    }

    pub fn name(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition,
    ) -> Self {
        self.name = Some(value);
        self
    }

    pub fn namespace(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition,
    ) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn version(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition,
    ) -> Self {
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

impl Default for ScaRequestDataAttributesDependenciesItemsLocationsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesDependenciesItemsLocationsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesDependenciesItemsLocationsItemsVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesDependenciesItemsLocationsItemsVisitor {
            type Value = ScaRequestDataAttributesDependenciesItemsLocationsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut block: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition> = None;
                let mut name: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition> = None;
                let mut namespace: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition> = None;
                let mut version: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "block" => {
                            if v.is_null() {
                                continue;
                            }
                            block = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ScaRequestDataAttributesDependenciesItemsLocationsItems {
                    block,
                    name,
                    namespace,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesDependenciesItemsLocationsItemsVisitor)
    }
}
