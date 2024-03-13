// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Finding object containing the finding information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BulkMuteFindingsRequestMetaFindings {
    /// The unique ID for this finding.
    #[serde(rename = "finding_id")]
    pub finding_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BulkMuteFindingsRequestMetaFindings {
    pub fn new() -> BulkMuteFindingsRequestMetaFindings {
        BulkMuteFindingsRequestMetaFindings {
            finding_id: None,
            _unparsed: false,
        }
    }

    pub fn finding_id(mut self, value: String) -> Self {
        self.finding_id = Some(value);
        self
    }
}

impl Default for BulkMuteFindingsRequestMetaFindings {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BulkMuteFindingsRequestMetaFindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BulkMuteFindingsRequestMetaFindingsVisitor;
        impl<'a> Visitor<'a> for BulkMuteFindingsRequestMetaFindingsVisitor {
            type Value = BulkMuteFindingsRequestMetaFindings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut finding_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "finding_id" => {
                            if v.is_null() {
                                continue;
                            }
                            finding_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = BulkMuteFindingsRequestMetaFindings {
                    finding_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BulkMuteFindingsRequestMetaFindingsVisitor)
    }
}
