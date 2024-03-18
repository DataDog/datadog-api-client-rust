// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The mute properties to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BulkMuteFindingsRequestAttributes {
    /// Object containing the new mute properties of the findings.
    #[serde(rename = "mute")]
    pub mute: crate::datadogV2::model::BulkMuteFindingsRequestProperties,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BulkMuteFindingsRequestAttributes {
    pub fn new(
        mute: crate::datadogV2::model::BulkMuteFindingsRequestProperties,
    ) -> BulkMuteFindingsRequestAttributes {
        BulkMuteFindingsRequestAttributes {
            mute,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for BulkMuteFindingsRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BulkMuteFindingsRequestAttributesVisitor;
        impl<'a> Visitor<'a> for BulkMuteFindingsRequestAttributesVisitor {
            type Value = BulkMuteFindingsRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut mute: Option<crate::datadogV2::model::BulkMuteFindingsRequestProperties> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "mute" => {
                            mute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let mute = mute.ok_or_else(|| M::Error::missing_field("mute"))?;

                let content = BulkMuteFindingsRequestAttributes { mute, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BulkMuteFindingsRequestAttributesVisitor)
    }
}
