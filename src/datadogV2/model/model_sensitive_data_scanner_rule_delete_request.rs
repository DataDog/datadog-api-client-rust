// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Delete rule request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerRuleDeleteRequest {
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerRuleDeleteRequest {
    pub fn new(
        meta: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> SensitiveDataScannerRuleDeleteRequest {
        SensitiveDataScannerRuleDeleteRequest {
            meta,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerRuleDeleteRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerRuleDeleteRequestVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerRuleDeleteRequestVisitor {
            type Value = SensitiveDataScannerRuleDeleteRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;

                let content = SensitiveDataScannerRuleDeleteRequest { meta, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerRuleDeleteRequestVisitor)
    }
}
