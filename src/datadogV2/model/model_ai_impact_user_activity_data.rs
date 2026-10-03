// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single daily AI tool activity entry.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIImpactUserActivityData {
    /// Daily AI coding tool activity for a single user. Each entry reports whether the user was
    /// active on a given day and which AI tools and models they used.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::AIImpactUserActivityAttributes,
    /// JSON:API type for AI Impact user activity entries.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AIImpactUserActivityType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIImpactUserActivityData {
    pub fn new(
        attributes: crate::datadogV2::model::AIImpactUserActivityAttributes,
        type_: crate::datadogV2::model::AIImpactUserActivityType,
    ) -> AIImpactUserActivityData {
        AIImpactUserActivityData {
            attributes,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for AIImpactUserActivityData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIImpactUserActivityDataVisitor;
        impl<'a> Visitor<'a> for AIImpactUserActivityDataVisitor {
            type Value = AIImpactUserActivityData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::AIImpactUserActivityAttributes,
                > = None;
                let mut type_: Option<crate::datadogV2::model::AIImpactUserActivityType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AIImpactUserActivityType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = AIImpactUserActivityData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIImpactUserActivityDataVisitor)
    }
}
