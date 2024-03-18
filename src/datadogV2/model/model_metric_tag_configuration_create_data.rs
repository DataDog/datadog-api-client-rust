// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object for a single metric to be configure tags on.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricTagConfigurationCreateData {
    /// Object containing the definition of a metric tag configuration to be created.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MetricTagConfigurationCreateAttributes>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: String,
    /// The metric tag configuration resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MetricTagConfigurationType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricTagConfigurationCreateData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::MetricTagConfigurationType,
    ) -> MetricTagConfigurationCreateData {
        MetricTagConfigurationCreateData {
            attributes: None,
            id,
            type_,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::MetricTagConfigurationCreateAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MetricTagConfigurationCreateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricTagConfigurationCreateDataVisitor;
        impl<'a> Visitor<'a> for MetricTagConfigurationCreateDataVisitor {
            type Value = MetricTagConfigurationCreateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::MetricTagConfigurationCreateAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::MetricTagConfigurationType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::MetricTagConfigurationType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = MetricTagConfigurationCreateData {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricTagConfigurationCreateDataVisitor)
    }
}
