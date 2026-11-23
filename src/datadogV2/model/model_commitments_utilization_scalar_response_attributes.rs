// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a scalar utilization response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsUtilizationScalarResponseAttributes {
    /// Array of scalar columns in the response.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV2::model::CommitmentsScalarColumn>,
    /// Metadata reporting that the cost data lacks the unused costs of some commitments, as with Azure exports
    /// scoped below the billing profile. Utilization is then overstated, and realized savings and effective
    /// savings rate are upper bounds. Omitted when the unused costs are available.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CommitmentsUnusedCostMeta>,
    /// Array of per-product utilization breakdown entries.
    #[serde(rename = "product_breakdown")]
    pub product_breakdown:
        Option<Vec<crate::datadogV2::model::CommitmentsUtilizationScalarProductBreakdownEntry>>,
    /// The query that produces the response values, in the same format as dashboard and notebook widget
    /// requests, so a client can graph the values in a widget. Each formula's `id` names the response value
    /// that it produces.
    #[serde(rename = "query_definition")]
    pub query_definition: Option<crate::datadogV2::model::CommitmentsQueryDefinition>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsUtilizationScalarResponseAttributes {
    pub fn new(
        columns: Vec<crate::datadogV2::model::CommitmentsScalarColumn>,
    ) -> CommitmentsUtilizationScalarResponseAttributes {
        CommitmentsUtilizationScalarResponseAttributes {
            columns,
            meta: None,
            product_breakdown: None,
            query_definition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::CommitmentsUnusedCostMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn product_breakdown(
        mut self,
        value: Vec<crate::datadogV2::model::CommitmentsUtilizationScalarProductBreakdownEntry>,
    ) -> Self {
        self.product_breakdown = Some(value);
        self
    }

    pub fn query_definition(
        mut self,
        value: crate::datadogV2::model::CommitmentsQueryDefinition,
    ) -> Self {
        self.query_definition = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsUtilizationScalarResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsUtilizationScalarResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CommitmentsUtilizationScalarResponseAttributesVisitor {
            type Value = CommitmentsUtilizationScalarResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV2::model::CommitmentsScalarColumn>> =
                    None;
                let mut meta: Option<crate::datadogV2::model::CommitmentsUnusedCostMeta> = None;
                let mut product_breakdown: Option<
                    Vec<crate::datadogV2::model::CommitmentsUtilizationScalarProductBreakdownEntry>,
                > = None;
                let mut query_definition: Option<
                    crate::datadogV2::model::CommitmentsQueryDefinition,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_breakdown" => {
                            if v.is_null() {
                                continue;
                            }
                            product_breakdown =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_definition" => {
                            if v.is_null() {
                                continue;
                            }
                            query_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;

                let content = CommitmentsUtilizationScalarResponseAttributes {
                    columns,
                    meta,
                    product_breakdown,
                    query_definition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsUtilizationScalarResponseAttributesVisitor)
    }
}
