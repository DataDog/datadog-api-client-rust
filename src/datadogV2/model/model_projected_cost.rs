// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Projected Cost data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProjectedCost {
    /// Projected Cost attributes data.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ProjectedCostAttributes>,
    /// Unique ID of the response.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of cost data.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ProjectedCostType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProjectedCost {
    pub fn new() -> ProjectedCost {
        ProjectedCost {
            attributes: None,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::ProjectedCostAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ProjectedCostType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ProjectedCost {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectedCost {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProjectedCostVisitor;
        impl<'a> Visitor<'a> for ProjectedCostVisitor {
            type Value = ProjectedCost;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::ProjectedCostAttributes> = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::ProjectedCostType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ProjectedCostType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = ProjectedCost {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProjectedCostVisitor)
    }
}
