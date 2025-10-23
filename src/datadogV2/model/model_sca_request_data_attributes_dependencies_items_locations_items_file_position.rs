// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A position in a file
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
    /// A position
    #[serde(rename = "end")]
    pub end: Option<
        crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition,
    >,
    /// The `file_name`.
    #[serde(rename = "file_name")]
    pub file_name: Option<String>,
    /// A position
    #[serde(rename = "start")]
    pub start: Option<
        crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
    pub fn new() -> ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
        ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
            end: None,
            file_name: None,
            start: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn end(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition,
    ) -> Self {
        self.end = Some(value);
        self
    }

    pub fn file_name(mut self, value: String) -> Self {
        self.file_name = Some(value);
        self
    }

    pub fn start(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition,
    ) -> Self {
        self.start = Some(value);
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

impl Default for ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePositionVisitor;
        impl<'a> Visitor<'a>
            for ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePositionVisitor
        {
            type Value = ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut end: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition> = None;
                let mut file_name: Option<String> = None;
                let mut start: Option<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItemsPosition> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "end" => {
                            if v.is_null() {
                                continue;
                            }
                            end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_name" => {
                            if v.is_null() {
                                continue;
                            }
                            file_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start" => {
                            if v.is_null() {
                                continue;
                            }
                            start = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePosition {
                    end,
                    file_name,
                    start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ScaRequestDataAttributesDependenciesItemsLocationsItemsFilePositionVisitor,
        )
    }
}
