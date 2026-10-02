// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of the experiment traffic summary.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsTrafficSummaryV2DTODataAttributes {
    /// Whether the observed variant traffic is imbalanced.
    #[serde(rename = "is_traffic_imbalanced")]
    pub is_traffic_imbalanced: Option<bool>,
    /// Total number of subjects included in the traffic summary.
    #[serde(rename = "total_subjects")]
    pub total_subjects: Option<i64>,
    /// Exposure counts for each experiment variant.
    #[serde(rename = "variants")]
    pub variants: Option<
        Vec<crate::datadogV2::model::ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsTrafficSummaryV2DTODataAttributes {
    pub fn new() -> ExperimentsTrafficSummaryV2DTODataAttributes {
        ExperimentsTrafficSummaryV2DTODataAttributes {
            is_traffic_imbalanced: None,
            total_subjects: None,
            variants: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_traffic_imbalanced(mut self, value: bool) -> Self {
        self.is_traffic_imbalanced = Some(value);
        self
    }

    pub fn total_subjects(mut self, value: i64) -> Self {
        self.total_subjects = Some(value);
        self
    }

    pub fn variants(
        mut self,
        value: Vec<
            crate::datadogV2::model::ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems,
        >,
    ) -> Self {
        self.variants = Some(value);
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

impl Default for ExperimentsTrafficSummaryV2DTODataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsTrafficSummaryV2DTODataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsTrafficSummaryV2DTODataAttributesVisitor;
        impl<'a> Visitor<'a> for ExperimentsTrafficSummaryV2DTODataAttributesVisitor {
            type Value = ExperimentsTrafficSummaryV2DTODataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_traffic_imbalanced: Option<bool> = None;
                let mut total_subjects: Option<i64> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_traffic_imbalanced" => {
                            if v.is_null() {
                                continue;
                            }
                            is_traffic_imbalanced =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_subjects" => {
                            if v.is_null() {
                                continue;
                            }
                            total_subjects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variants" => {
                            if v.is_null() {
                                continue;
                            }
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsTrafficSummaryV2DTODataAttributes {
                    is_traffic_imbalanced,
                    total_subjects,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsTrafficSummaryV2DTODataAttributesVisitor)
    }
}
