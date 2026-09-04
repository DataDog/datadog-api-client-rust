// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a unit cost create or replace request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UnitCostRequestAttributes {
    /// A timeseries object containing `queries` and `formulas` arrays.
    #[serde(rename = "denominator_query")]
    pub denominator_query: crate::datadogV2::model::UnitCostQueryDefinition,
    /// An optional description of the unit cost. At most 2000 characters.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The name of the unit cost. At most 200 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// A timeseries object containing `queries` and `formulas` arrays.
    #[serde(rename = "numerator_query")]
    pub numerator_query: crate::datadogV2::model::UnitCostQueryDefinition,
    /// The label describing the denominator unit, for example `user`. At most 100 characters.
    #[serde(rename = "unit_label")]
    pub unit_label: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UnitCostRequestAttributes {
    pub fn new(
        denominator_query: crate::datadogV2::model::UnitCostQueryDefinition,
        name: String,
        numerator_query: crate::datadogV2::model::UnitCostQueryDefinition,
        unit_label: String,
    ) -> UnitCostRequestAttributes {
        UnitCostRequestAttributes {
            denominator_query,
            description: None,
            name,
            numerator_query,
            unit_label,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for UnitCostRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnitCostRequestAttributesVisitor;
        impl<'a> Visitor<'a> for UnitCostRequestAttributesVisitor {
            type Value = UnitCostRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut denominator_query: Option<
                    crate::datadogV2::model::UnitCostQueryDefinition,
                > = None;
                let mut description: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut numerator_query: Option<crate::datadogV2::model::UnitCostQueryDefinition> =
                    None;
                let mut unit_label: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "denominator_query" => {
                            denominator_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "numerator_query" => {
                            numerator_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit_label" => {
                            unit_label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let denominator_query = denominator_query
                    .ok_or_else(|| M::Error::missing_field("denominator_query"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let numerator_query =
                    numerator_query.ok_or_else(|| M::Error::missing_field("numerator_query"))?;
                let unit_label = unit_label.ok_or_else(|| M::Error::missing_field("unit_label"))?;

                let content = UnitCostRequestAttributes {
                    denominator_query,
                    description,
                    name,
                    numerator_query,
                    unit_label,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UnitCostRequestAttributesVisitor)
    }
}
