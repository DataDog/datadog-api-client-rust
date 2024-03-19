// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request object that includes the metric that you would like to configure tags for.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricTagConfigurationCreateRequest {
    /// Object for a single metric to be configure tags on.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::MetricTagConfigurationCreateData,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricTagConfigurationCreateRequest {
    pub fn new(
        data: crate::datadogV2::model::MetricTagConfigurationCreateData,
    ) -> MetricTagConfigurationCreateRequest {
        MetricTagConfigurationCreateRequest {
            data,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MetricTagConfigurationCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricTagConfigurationCreateRequestVisitor;
        impl<'a> Visitor<'a> for MetricTagConfigurationCreateRequestVisitor {
            type Value = MetricTagConfigurationCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::MetricTagConfigurationCreateData> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = MetricTagConfigurationCreateRequest { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricTagConfigurationCreateRequestVisitor)
    }
}
