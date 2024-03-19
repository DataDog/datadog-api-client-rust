// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update group response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerGroupUpdateResponse {
    /// Meta payload containing information about the API.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerGroupUpdateResponse {
    pub fn new() -> SensitiveDataScannerGroupUpdateResponse {
        SensitiveDataScannerGroupUpdateResponse {
            meta: None,
            _unparsed: false,
        }
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerMetaVersionOnly,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerGroupUpdateResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerGroupUpdateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerGroupUpdateResponseVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerGroupUpdateResponseVisitor {
            type Value = SensitiveDataScannerGroupUpdateResponse;

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
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerGroupUpdateResponse { meta, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerGroupUpdateResponseVisitor)
    }
}
