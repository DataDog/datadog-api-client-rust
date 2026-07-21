// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Top-level JSON:API meta block accompanying every DDSQL tabular query response.
/// Carries standard observability handles for client-side correlation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DdsqlTabularQueryResponseMeta {
    /// Server-side time spent serving this request, in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: i64,
    /// Echo of the `DD-Request-ID` header assigned by Datadog's edge to this request,
    /// for support correlation.
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DdsqlTabularQueryResponseMeta {
    pub fn new(elapsed: i64, request_id: String) -> DdsqlTabularQueryResponseMeta {
        DdsqlTabularQueryResponseMeta {
            elapsed,
            request_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DdsqlTabularQueryResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DdsqlTabularQueryResponseMetaVisitor;
        impl<'a> Visitor<'a> for DdsqlTabularQueryResponseMetaVisitor {
            type Value = DdsqlTabularQueryResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut elapsed: Option<i64> = None;
                let mut request_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "elapsed" => {
                            elapsed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "request_id" => {
                            request_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let elapsed = elapsed.ok_or_else(|| M::Error::missing_field("elapsed"))?;
                let request_id = request_id.ok_or_else(|| M::Error::missing_field("request_id"))?;

                let content = DdsqlTabularQueryResponseMeta {
                    elapsed,
                    request_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DdsqlTabularQueryResponseMetaVisitor)
    }
}
