// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Batch encoding configuration for the ClickHouse destination.
/// Required when `format` is `arrow_stream`. The `codec` field must be set to `arrow_stream`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineClickhouseDestinationBatchEncoding {
    /// When `true`, null values are allowed for non-nullable fields in the ClickHouse schema.
    /// When `false` (default), missing values for non-nullable columns cause encoding errors.
    #[serde(rename = "allow_nullable_fields")]
    pub allow_nullable_fields: Option<bool>,
    /// The codec used for batch encoding. Only `arrow_stream` is supported.
    #[serde(rename = "codec")]
    pub codec:
        crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncodingCodec,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineClickhouseDestinationBatchEncoding {
    pub fn new(
        codec: crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncodingCodec,
    ) -> ObservabilityPipelineClickhouseDestinationBatchEncoding {
        ObservabilityPipelineClickhouseDestinationBatchEncoding {
            allow_nullable_fields: None,
            codec,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_nullable_fields(mut self, value: bool) -> Self {
        self.allow_nullable_fields = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineClickhouseDestinationBatchEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineClickhouseDestinationBatchEncodingVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineClickhouseDestinationBatchEncodingVisitor {
            type Value = ObservabilityPipelineClickhouseDestinationBatchEncoding;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_nullable_fields: Option<bool> = None;
                let mut codec: Option<crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncodingCodec> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_nullable_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_nullable_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "codec" => {
                            codec = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _codec) = codec {
                                match _codec {
                                    crate::datadogV2::model::ObservabilityPipelineClickhouseDestinationBatchEncodingCodec::UnparsedObject(_codec) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let codec = codec.ok_or_else(|| M::Error::missing_field("codec"))?;

                let content = ObservabilityPipelineClickhouseDestinationBatchEncoding {
                    allow_nullable_fields,
                    codec,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineClickhouseDestinationBatchEncodingVisitor)
    }
}
