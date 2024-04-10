// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The data portion of the SLO report request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SloReportCreateRequestData {
    /// The attributes portion of the SLO report request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SloReportCreateRequestAttributes,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SloReportCreateRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::SloReportCreateRequestAttributes,
    ) -> SloReportCreateRequestData {
        SloReportCreateRequestData {
            attributes,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SloReportCreateRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SloReportCreateRequestDataVisitor;
        impl<'a> Visitor<'a> for SloReportCreateRequestDataVisitor {
            type Value = SloReportCreateRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::SloReportCreateRequestAttributes,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;

                let content = SloReportCreateRequestData {
                    attributes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SloReportCreateRequestDataVisitor)
    }
}
