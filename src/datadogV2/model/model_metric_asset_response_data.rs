// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metric assets response data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricAssetResponseData {
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: String,
    /// Relationships to assets related to the metric.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::MetricAssetResponseRelationships>,
    /// The metric resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MetricType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricAssetResponseData {
    pub fn new(id: String, type_: crate::datadogV2::model::MetricType) -> MetricAssetResponseData {
        MetricAssetResponseData {
            id,
            relationships: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::MetricAssetResponseRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MetricAssetResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricAssetResponseDataVisitor;
        impl<'a> Visitor<'a> for MetricAssetResponseDataVisitor {
            type Value = MetricAssetResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut relationships: Option<
                    crate::datadogV2::model::MetricAssetResponseRelationships,
                > = None;
                let mut type_: Option<crate::datadogV2::model::MetricType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationships" => {
                            if v.is_null() {
                                continue;
                            }
                            relationships =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::MetricType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = MetricAssetResponseData {
                    id,
                    relationships,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricAssetResponseDataVisitor)
    }
}
