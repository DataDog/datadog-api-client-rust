// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mobile application for mobile synthetics test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileTestsMobileApplication {
    /// Application ID of the mobile application.
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    /// Reference ID of the mobile application.
    #[serde(rename = "referenceId")]
    pub reference_id: Option<String>,
    /// Reference type for the mobile application for a mobile synthetics test.
    #[serde(rename = "referenceType")]
    pub reference_type:
        Option<crate::datadogV1::model::SyntheticsMobileTestsMobileApplicationReferenceType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileTestsMobileApplication {
    pub fn new() -> SyntheticsMobileTestsMobileApplication {
        SyntheticsMobileTestsMobileApplication {
            application_id: None,
            reference_id: None,
            reference_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: String) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn reference_id(mut self, value: String) -> Self {
        self.reference_id = Some(value);
        self
    }

    pub fn reference_type(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestsMobileApplicationReferenceType,
    ) -> Self {
        self.reference_type = Some(value);
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

impl Default for SyntheticsMobileTestsMobileApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileTestsMobileApplication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileTestsMobileApplicationVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileTestsMobileApplicationVisitor {
            type Value = SyntheticsMobileTestsMobileApplication;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut reference_id: Option<String> = None;
                let mut reference_type: Option<
                    crate::datadogV1::model::SyntheticsMobileTestsMobileApplicationReferenceType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "applicationId" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "referenceId" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "referenceType" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _reference_type) = reference_type {
                                match _reference_type {
                                    crate::datadogV1::model::SyntheticsMobileTestsMobileApplicationReferenceType::UnparsedObject(_reference_type) => {
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

                let content = SyntheticsMobileTestsMobileApplication {
                    application_id,
                    reference_id,
                    reference_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileTestsMobileApplicationVisitor)
    }
}
