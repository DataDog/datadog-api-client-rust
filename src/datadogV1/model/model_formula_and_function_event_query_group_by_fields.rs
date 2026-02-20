// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Flat group by using multiple event facets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FormulaAndFunctionEventQueryGroupByFields {
    /// List of event facets to group by.
    #[serde(rename = "fields")]
    pub fields: Vec<String>,
    /// Number of groups to return.
    #[serde(rename = "limit")]
    pub limit: Option<i64>,
    /// Options for sorting group by results.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FormulaAndFunctionEventQueryGroupByFields {
    pub fn new(fields: Vec<String>) -> FormulaAndFunctionEventQueryGroupByFields {
        FormulaAndFunctionEventQueryGroupByFields {
            fields,
            limit: None,
            sort: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort,
    ) -> Self {
        self.sort = Some(value);
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

impl<'de> Deserialize<'de> for FormulaAndFunctionEventQueryGroupByFields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FormulaAndFunctionEventQueryGroupByFieldsVisitor;
        impl<'a> Visitor<'a> for FormulaAndFunctionEventQueryGroupByFieldsVisitor {
            type Value = FormulaAndFunctionEventQueryGroupByFields;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fields: Option<Vec<String>> = None;
                let mut limit: Option<i64> = None;
                let mut sort: Option<
                    crate::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fields" => {
                            fields = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            if v.is_null() {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sort" => {
                            if v.is_null() {
                                continue;
                            }
                            sort = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let fields = fields.ok_or_else(|| M::Error::missing_field("fields"))?;

                let content = FormulaAndFunctionEventQueryGroupByFields {
                    fields,
                    limit,
                    sort,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FormulaAndFunctionEventQueryGroupByFieldsVisitor)
    }
}
