// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DORAIncidentRequestData {
    /// Attributes to create a DORA incident event.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::DORAIncidentRequestAttributes,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DORAIncidentRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::DORAIncidentRequestAttributes,
    ) -> DORAIncidentRequestData {
        DORAIncidentRequestData {
            attributes,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DORAIncidentRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DORAIncidentRequestDataVisitor;
        impl<'a> Visitor<'a> for DORAIncidentRequestDataVisitor {
            type Value = DORAIncidentRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::DORAIncidentRequestAttributes> =
                    None;
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

                let content = DORAIncidentRequestData {
                    attributes,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DORAIncidentRequestDataVisitor)
    }
}
