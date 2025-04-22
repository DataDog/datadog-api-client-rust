// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact {
    /// The definition of `ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactAction` object.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactAction,
    /// The definition of `ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions` object.
    #[serde(rename = "options")]
    pub options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact {
    pub fn new(
        action: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactAction,
        options: crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions,
    ) -> ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact {
        ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact {
            action,
            options,
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
    for ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactVisitor;
        impl<'a> Visitor<'a>
            for ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactVisitor
        {
            type Value = ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactAction> = None;
                let mut options: Option<crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactOptions> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactAction::UnparsedObject(_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;

                let content =
                    ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedact {
                        action,
                        options,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ObservabilityPipelineSensitiveDataScannerProcessorActionPartialRedactVisitor,
        )
    }
}
