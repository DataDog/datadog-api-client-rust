// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an aggregated DNS flow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SingleAggregatedDnsResponseDataAttributes {
    /// The key, value pairs for each group by.
    #[serde(rename = "group_bys")]
    pub group_bys:
        Option<Vec<crate::datadogV2::model::SingleAggregatedDnsResponseDataAttributesGroupByItems>>,
    /// Metrics associated with an aggregated DNS flow.
    #[serde(rename = "metrics")]
    pub metrics:
        Option<Vec<crate::datadogV2::model::SingleAggregatedDnsResponseDataAttributesMetricsItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SingleAggregatedDnsResponseDataAttributes {
    pub fn new() -> SingleAggregatedDnsResponseDataAttributes {
        SingleAggregatedDnsResponseDataAttributes {
            group_bys: None,
            metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group_bys(
        mut self,
        value: Vec<crate::datadogV2::model::SingleAggregatedDnsResponseDataAttributesGroupByItems>,
    ) -> Self {
        self.group_bys = Some(value);
        self
    }

    pub fn metrics(
        mut self,
        value: Vec<crate::datadogV2::model::SingleAggregatedDnsResponseDataAttributesMetricsItems>,
    ) -> Self {
        self.metrics = Some(value);
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

impl Default for SingleAggregatedDnsResponseDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SingleAggregatedDnsResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SingleAggregatedDnsResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for SingleAggregatedDnsResponseDataAttributesVisitor {
            type Value = SingleAggregatedDnsResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_bys: Option<Vec<crate::datadogV2::model::SingleAggregatedDnsResponseDataAttributesGroupByItems>> = None;
                let mut metrics: Option<Vec<crate::datadogV2::model::SingleAggregatedDnsResponseDataAttributesMetricsItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_bys" => {
                            if v.is_null() {
                                continue;
                            }
                            group_bys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SingleAggregatedDnsResponseDataAttributes {
                    group_bys,
                    metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SingleAggregatedDnsResponseDataAttributesVisitor)
    }
}
