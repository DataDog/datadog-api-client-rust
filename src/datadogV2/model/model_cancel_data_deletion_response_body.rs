// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response from the cancel data deletion request endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CancelDataDeletionResponseBody {
    /// The created data deletion request information.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::DataDeletionResponseItem>,
    /// The metadata of the data deletion response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::DataDeletionResponseMeta>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CancelDataDeletionResponseBody {
    pub fn new() -> CancelDataDeletionResponseBody {
        CancelDataDeletionResponseBody {
            data: None,
            meta: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::DataDeletionResponseItem) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::DataDeletionResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for CancelDataDeletionResponseBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CancelDataDeletionResponseBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CancelDataDeletionResponseBodyVisitor;
        impl<'a> Visitor<'a> for CancelDataDeletionResponseBodyVisitor {
            type Value = CancelDataDeletionResponseBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::DataDeletionResponseItem> = None;
                let mut meta: Option<crate::datadogV2::model::DataDeletionResponseMeta> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CancelDataDeletionResponseBody {
                    data,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CancelDataDeletionResponseBodyVisitor)
    }
}
