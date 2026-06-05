// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single validation issue found while validating an AWS Cost and Usage Report (CUR) 2.0 configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSCcmConfigValidationIssue {
    /// Identifies the specific reason a Cost and Usage Report (CUR) 2.0 configuration failed validation.
    #[serde(rename = "code")]
    pub code: crate::datadogV2::model::AWSCcmConfigValidationIssueCode,
    /// Human-readable description of the validation issue.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSCcmConfigValidationIssue {
    pub fn new(
        code: crate::datadogV2::model::AWSCcmConfigValidationIssueCode,
        description: String,
    ) -> AWSCcmConfigValidationIssue {
        AWSCcmConfigValidationIssue {
            code,
            description,
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

impl<'de> Deserialize<'de> for AWSCcmConfigValidationIssue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSCcmConfigValidationIssueVisitor;
        impl<'a> Visitor<'a> for AWSCcmConfigValidationIssueVisitor {
            type Value = AWSCcmConfigValidationIssue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut code: Option<crate::datadogV2::model::AWSCcmConfigValidationIssueCode> =
                    None;
                let mut description: Option<String> = None;
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
                                    crate::datadogV2::model::AWSCcmConfigValidationIssueCode::UnparsedObject(_code) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let code = code.ok_or_else(|| M::Error::missing_field("code"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;

                let content = AWSCcmConfigValidationIssue {
                    code,
                    description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSCcmConfigValidationIssueVisitor)
    }
}
