// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a commitments list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsListResponseAttributes {
    /// Array of commitment items.
    #[serde(rename = "commitments")]
    pub commitments: Vec<crate::datadogV2::model::CommitmentsListItem>,
    /// Metadata for a commitments list response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CommitmentsListMeta>,
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

impl CommitmentsListResponseAttributes {
    pub fn new(
        commitments: Vec<crate::datadogV2::model::CommitmentsListItem>,
    ) -> CommitmentsListResponseAttributes {
        CommitmentsListResponseAttributes {
            commitments,
            meta: None,
            query_definition: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(mut self, value: crate::datadogV2::model::CommitmentsListMeta) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsListResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsListResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CommitmentsListResponseAttributesVisitor {
            type Value = CommitmentsListResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commitments: Option<Vec<crate::datadogV2::model::CommitmentsListItem>> =
                    None;
                let mut meta: Option<crate::datadogV2::model::CommitmentsListMeta> = None;
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
                        "commitments" => {
                            commitments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let commitments =
                    commitments.ok_or_else(|| M::Error::missing_field("commitments"))?;

                let content = CommitmentsListResponseAttributes {
                    commitments,
                    meta,
                    query_definition,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsListResponseAttributesVisitor)
    }
}
