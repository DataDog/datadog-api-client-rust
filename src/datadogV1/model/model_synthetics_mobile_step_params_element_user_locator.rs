// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// User locator to find the element.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileStepParamsElementUserLocator {
    /// Whether if the test should fail if the element cannot be found.
    #[serde(rename = "failTestOnCannotLocate")]
    pub fail_test_on_cannot_locate: Option<bool>,
    /// Values of the user locator.
    #[serde(rename = "values")]
    pub values: Option<
        Vec<crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItems>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileStepParamsElementUserLocator {
    pub fn new() -> SyntheticsMobileStepParamsElementUserLocator {
        SyntheticsMobileStepParamsElementUserLocator {
            fail_test_on_cannot_locate: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn fail_test_on_cannot_locate(mut self, value: bool) -> Self {
        self.fail_test_on_cannot_locate = Some(value);
        self
    }

    pub fn values(
        mut self,
        value: Vec<
            crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItems,
        >,
    ) -> Self {
        self.values = Some(value);
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

impl Default for SyntheticsMobileStepParamsElementUserLocator {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileStepParamsElementUserLocator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileStepParamsElementUserLocatorVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileStepParamsElementUserLocatorVisitor {
            type Value = SyntheticsMobileStepParamsElementUserLocator;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fail_test_on_cannot_locate: Option<bool> = None;
                let mut values: Option<Vec<crate::datadogV1::model::SyntheticsMobileStepParamsElementUserLocatorValuesItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failTestOnCannotLocate" => {
                            if v.is_null() {
                                continue;
                            }
                            fail_test_on_cannot_locate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileStepParamsElementUserLocator {
                    fail_test_on_cannot_locate,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileStepParamsElementUserLocatorVisitor)
    }
}
