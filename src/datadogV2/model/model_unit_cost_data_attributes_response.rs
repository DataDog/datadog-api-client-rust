// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a unit cost.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UnitCostDataAttributesResponse {
    /// The time the unit cost was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who created the unit cost.
    #[serde(rename = "created_by")]
    pub created_by: uuid::Uuid,
    /// A timeseries object containing `queries` and `formulas` arrays.
    #[serde(rename = "denominator_query")]
    pub denominator_query: crate::datadogV2::model::UnitCostQueryDefinition,
    /// The data source of the denominator queries, or `multisource` when the denominator
    /// queries span more than one data source.
    #[serde(rename = "denominator_type")]
    pub denominator_type: String,
    /// The description of the unit cost. Omitted when the unit cost has no description.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// The name of the unit cost.
    #[serde(rename = "name")]
    pub name: String,
    /// A timeseries object containing `queries` and `formulas` arrays.
    #[serde(rename = "numerator_query")]
    pub numerator_query: crate::datadogV2::model::UnitCostQueryDefinition,
    /// The ID of the organization the unit cost belongs to.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The label describing the denominator unit.
    #[serde(rename = "unit_label")]
    pub unit_label: String,
    /// The time the unit cost was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who last updated the unit cost.
    #[serde(rename = "updated_by")]
    pub updated_by: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UnitCostDataAttributesResponse {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: uuid::Uuid,
        denominator_query: crate::datadogV2::model::UnitCostQueryDefinition,
        denominator_type: String,
        name: String,
        numerator_query: crate::datadogV2::model::UnitCostQueryDefinition,
        org_id: i64,
        unit_label: String,
        updated_at: chrono::DateTime<chrono::Utc>,
        updated_by: uuid::Uuid,
    ) -> UnitCostDataAttributesResponse {
        UnitCostDataAttributesResponse {
            created_at,
            created_by,
            denominator_query,
            denominator_type,
            description: None,
            name,
            numerator_query,
            org_id,
            unit_label,
            updated_at,
            updated_by,
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

impl<'de> Deserialize<'de> for UnitCostDataAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UnitCostDataAttributesResponseVisitor;
        impl<'a> Visitor<'a> for UnitCostDataAttributesResponseVisitor {
            type Value = UnitCostDataAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
                let mut denominator_query: Option<
                    crate::datadogV2::model::UnitCostQueryDefinition,
                > = None;
                let mut denominator_type: Option<String> = None;
                let mut description: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut numerator_query: Option<crate::datadogV2::model::UnitCostQueryDefinition> =
                    None;
                let mut org_id: Option<i64> = None;
                let mut unit_label: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut updated_by: Option<uuid::Uuid> = None;
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
                        "denominator_query" => {
                            denominator_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "denominator_type" => {
                            denominator_type =
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
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unit_label" => {
                            unit_label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let denominator_query = denominator_query
                    .ok_or_else(|| M::Error::missing_field("denominator_query"))?;
                let denominator_type =
                    denominator_type.ok_or_else(|| M::Error::missing_field("denominator_type"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let numerator_query =
                    numerator_query.ok_or_else(|| M::Error::missing_field("numerator_query"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let unit_label = unit_label.ok_or_else(|| M::Error::missing_field("unit_label"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;

                let content = UnitCostDataAttributesResponse {
                    created_at,
                    created_by,
                    denominator_query,
                    denominator_type,
                    description,
                    name,
                    numerator_query,
                    org_id,
                    unit_label,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UnitCostDataAttributesResponseVisitor)
    }
}
