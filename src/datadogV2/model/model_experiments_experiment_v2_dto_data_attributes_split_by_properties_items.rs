// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Property used to split experiment results into analysis dimensions.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems {
    /// Exposure field or Warehouse column used for the analysis dimension.
    #[serde(rename = "column_name")]
    pub column_name: String,
    /// Type of the Datadog exposure field or Warehouse column.
    #[serde(rename = "column_type")]
    pub column_type: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType,
    /// Read-only property ID. Omit it from POST and PATCH; writes identify properties by column_name.
    #[serde(rename = "id")]
    pub id: String,
    /// Display name for the analysis dimension.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems {
    pub fn new(
        column_name: String,
        column_type: crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType,
        id: String,
        name: String,
    ) -> ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems {
        ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems {
            column_name,
            column_type,
            id,
            name,
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

impl<'de> Deserialize<'de> for ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItemsVisitor {
            type Value = ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut column_name: Option<String> = None;
                let mut column_type: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType> = None;
                let mut id: Option<String> = None;
                let mut name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "column_name" => {
                            column_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column_type" => {
                            column_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _column_type) = column_type {
                                match _column_type {
                                    crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesSplitByPropertiesItemsColumnType::UnparsedObject(_column_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let column_name =
                    column_name.ok_or_else(|| M::Error::missing_field("column_name"))?;
                let column_type =
                    column_type.ok_or_else(|| M::Error::missing_field("column_type"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItems {
                    column_name,
                    column_type,
                    id,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsExperimentV2DTODataAttributesSplitByPropertiesItemsVisitor)
    }
}
