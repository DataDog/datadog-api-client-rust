// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Case update status attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseUpdateStatusAttributes {
    /// Case status
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::CaseStatus,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseUpdateStatusAttributes {
    pub fn new(status: crate::datadogV2::model::CaseStatus) -> CaseUpdateStatusAttributes {
        CaseUpdateStatusAttributes {
            status,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CaseUpdateStatusAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseUpdateStatusAttributesVisitor;
        impl<'a> Visitor<'a> for CaseUpdateStatusAttributesVisitor {
            type Value = CaseUpdateStatusAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut status: Option<crate::datadogV2::model::CaseStatus> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CaseStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = CaseUpdateStatusAttributes { status, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseUpdateStatusAttributesVisitor)
    }
}
