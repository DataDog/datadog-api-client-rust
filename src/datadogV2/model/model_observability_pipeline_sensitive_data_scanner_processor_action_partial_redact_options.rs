// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Controls how partial redaction is applied, including character count and direction.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions {
    /// The `ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions` `characters`.
    #[serde(rename = "characters")]
    pub characters: i64,
    /// Indicates whether to redact characters from the first or last part of the matched value.
    #[serde(rename = "direction")]
    pub direction: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsDirection,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions {
    pub fn new(
        characters: i64,
        direction: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsDirection,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions {
        ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions {
            characters,
            direction,
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

impl<'de> Deserialize<'de>
    for ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsVisitor;
        impl<'a> Visitor<'a>
            for ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsVisitor
        {
            type Value =
                ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut characters: Option<i64> = None;
                let mut direction: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsDirection> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "characters" => {
                            characters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "direction" => {
                            direction = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _direction) = direction {
                                match _direction {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsDirection::UnparsedObject(_direction) => {
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
                let characters = characters.ok_or_else(|| M::Error::missing_field("characters"))?;
                let direction = direction.ok_or_else(|| M::Error::missing_field("direction"))?;

                let content =
                    ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions {
                        characters,
                        direction,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptionsVisitor,
        )
    }
}
