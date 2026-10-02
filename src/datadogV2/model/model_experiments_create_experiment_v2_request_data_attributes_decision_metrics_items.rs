// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metric used to make an experiment decision, with its primary metric designation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems {
    /// Whether this is the experiment primary metric.
    #[serde(rename = "is_primary")]
    pub is_primary: bool,
    /// Decision metric UUID.
    #[serde(rename = "metric_id")]
    pub metric_id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems {
    pub fn new(
        is_primary: bool,
        metric_id: uuid::Uuid,
    ) -> ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems {
        ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems {
            is_primary,
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
    for ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItemsVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItemsVisitor
        {
            type Value = ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_primary: Option<bool> = None;
                let mut metric_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_primary" => {
                            is_primary = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                let is_primary = is_primary.ok_or_else(|| M::Error::missing_field("is_primary"))?;
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;

                let content =
                    ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItems {
                        is_primary,
                        metric_id,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsCreateExperimentV2RequestDataAttributesDecisionMetricsItemsVisitor,
        )
    }
}
