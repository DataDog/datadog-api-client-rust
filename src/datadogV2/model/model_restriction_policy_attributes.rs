// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Restriction policy attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RestrictionPolicyAttributes {
    /// An array of bindings.
    #[serde(rename = "bindings")]
    pub bindings: Vec<crate::datadogV2::model::RestrictionPolicyBinding>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RestrictionPolicyAttributes {
    pub fn new(
        bindings: Vec<crate::datadogV2::model::RestrictionPolicyBinding>,
    ) -> RestrictionPolicyAttributes {
        RestrictionPolicyAttributes {
            bindings,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RestrictionPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RestrictionPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for RestrictionPolicyAttributesVisitor {
            type Value = RestrictionPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bindings: Option<Vec<crate::datadogV2::model::RestrictionPolicyBinding>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bindings" => {
                            bindings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let bindings = bindings.ok_or_else(|| M::Error::missing_field("bindings"))?;

                let content = RestrictionPolicyAttributes {
                    bindings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RestrictionPolicyAttributesVisitor)
    }
}
