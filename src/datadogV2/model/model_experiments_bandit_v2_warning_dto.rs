// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A warning about the response that isn't an error.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditV2WarningDTO {
    /// Code that identifies the warning. `datadog_flag_configuration_omitted` means the caller lacks feature flag or environment read permissions, so the response leaves out `datadog_flag_configuration`.
    #[serde(rename = "code")]
    pub code: crate::datadogV2::model::ExperimentsBanditV2WarningCode,
    /// Explanation of the warning.
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditV2WarningDTO {
    pub fn new(
        code: crate::datadogV2::model::ExperimentsBanditV2WarningCode,
        detail: String,
    ) -> ExperimentsBanditV2WarningDTO {
        ExperimentsBanditV2WarningDTO {
            code,
            detail,
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

impl<'de> Deserialize<'de> for ExperimentsBanditV2WarningDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditV2WarningDTOVisitor;
        impl<'a> Visitor<'a> for ExperimentsBanditV2WarningDTOVisitor {
            type Value = ExperimentsBanditV2WarningDTO;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<crate::datadogV2::model::ExperimentsBanditV2WarningCode> =
                    None;
                let mut detail: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "code" => {
                            code = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _code) = code {
                                match _code {
                                    crate::datadogV2::model::ExperimentsBanditV2WarningCode::UnparsedObject(_code) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "detail" => {
                            detail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let detail = detail.ok_or_else(|| M::Error::missing_field("detail"))?;

                let content = ExperimentsBanditV2WarningDTO {
                    code,
                    detail,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditV2WarningDTOVisitor)
    }
}
