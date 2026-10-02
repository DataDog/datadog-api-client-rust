// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Reference to a metric to include in the collection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems {
    /// Identifier of the metric to include in the collection.
    #[serde(rename = "metric_id")]
    pub metric_id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems {
    pub fn new(
        metric_id: uuid::Uuid,
    ) -> ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems {
        ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems {
            metric_id,
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

impl<'de> Deserialize<'de>
    for ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItemsVisitor
        {
            type Value = ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metric_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;

                let content =
                    ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItems {
                        metric_id,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsCreateMetricCollectionV2RequestDataAttributesMetricsItemsVisitor,
        )
    }
}
