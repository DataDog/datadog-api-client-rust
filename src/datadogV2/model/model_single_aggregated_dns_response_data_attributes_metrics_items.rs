// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metrics associated with an aggregated DNS flow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SingleAggregatedDnsResponseDataAttributesMetricsItems {
    /// The metric key for DNS metrics.
    #[serde(rename = "key")]
    pub key: Option<crate::datadogV2::model::DnsMetricKey>,
    /// The metric value.
    #[serde(rename = "value")]
    pub value: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SingleAggregatedDnsResponseDataAttributesMetricsItems {
    pub fn new() -> SingleAggregatedDnsResponseDataAttributesMetricsItems {
        SingleAggregatedDnsResponseDataAttributesMetricsItems {
            key: None,
            value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn key(mut self, value: crate::datadogV2::model::DnsMetricKey) -> Self {
        self.key = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
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

impl Default for SingleAggregatedDnsResponseDataAttributesMetricsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SingleAggregatedDnsResponseDataAttributesMetricsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SingleAggregatedDnsResponseDataAttributesMetricsItemsVisitor;
        impl<'a> Visitor<'a> for SingleAggregatedDnsResponseDataAttributesMetricsItemsVisitor {
            type Value = SingleAggregatedDnsResponseDataAttributesMetricsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut key: Option<crate::datadogV2::model::DnsMetricKey> = None;
                let mut value: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _key) = key {
                                match _key {
                                    crate::datadogV2::model::DnsMetricKey::UnparsedObject(_key) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SingleAggregatedDnsResponseDataAttributesMetricsItems {
                    key,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SingleAggregatedDnsResponseDataAttributesMetricsItemsVisitor)
    }
}
