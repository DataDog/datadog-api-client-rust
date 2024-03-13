// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Optional parameters for bulk deleting metric tag configurations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricBulkTagConfigDeleteAttributes {
    /// A list of account emails to notify when the configuration is applied.
    #[serde(rename = "emails")]
    pub emails: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricBulkTagConfigDeleteAttributes {
    pub fn new() -> MetricBulkTagConfigDeleteAttributes {
        MetricBulkTagConfigDeleteAttributes {
            emails: None,
            _unparsed: false,
        }
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }
}

impl Default for MetricBulkTagConfigDeleteAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricBulkTagConfigDeleteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricBulkTagConfigDeleteAttributesVisitor;
        impl<'a> Visitor<'a> for MetricBulkTagConfigDeleteAttributesVisitor {
            type Value = MetricBulkTagConfigDeleteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut emails: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "emails" => {
                            if v.is_null() {
                                continue;
                            }
                            emails = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricBulkTagConfigDeleteAttributes { emails, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricBulkTagConfigDeleteAttributesVisitor)
    }
}
