// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The distribution points payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DistributionPointsPayload {
    /// A list of distribution points series to submit to Datadog.
    #[serde(rename = "series")]
    pub series: Vec<crate::datadogV1::model::DistributionPointsSeries>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DistributionPointsPayload {
    pub fn new(
        series: Vec<crate::datadogV1::model::DistributionPointsSeries>,
    ) -> DistributionPointsPayload {
        DistributionPointsPayload {
            series,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DistributionPointsPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionPointsPayloadVisitor;
        impl<'a> Visitor<'a> for DistributionPointsPayloadVisitor {
            type Value = DistributionPointsPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut series: Option<Vec<crate::datadogV1::model::DistributionPointsSeries>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "series" => {
                            series = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let series = series.ok_or_else(|| M::Error::missing_field("series"))?;

                let content = DistributionPointsPayload { series, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionPointsPayloadVisitor)
    }
}
