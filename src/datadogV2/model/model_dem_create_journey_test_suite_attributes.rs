// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a test suite for a DEM journey.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DemCreateJourneyTestSuiteAttributes {
    /// Whether to populate the test suite based on journey coverage data.
    #[serde(
        rename = "include_tests_from_journey_coverage",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub include_tests_from_journey_coverage: Option<Option<bool>>,
    /// An optional custom name for the auto-created test suite.
    #[serde(
        rename = "test_suite_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub test_suite_name: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DemCreateJourneyTestSuiteAttributes {
    pub fn new() -> DemCreateJourneyTestSuiteAttributes {
        DemCreateJourneyTestSuiteAttributes {
            include_tests_from_journey_coverage: None,
            test_suite_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn include_tests_from_journey_coverage(mut self, value: Option<bool>) -> Self {
        self.include_tests_from_journey_coverage = Some(value);
        self
    }

    pub fn test_suite_name(mut self, value: Option<String>) -> Self {
        self.test_suite_name = Some(value);
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

impl Default for DemCreateJourneyTestSuiteAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DemCreateJourneyTestSuiteAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DemCreateJourneyTestSuiteAttributesVisitor;
        impl<'a> Visitor<'a> for DemCreateJourneyTestSuiteAttributesVisitor {
            type Value = DemCreateJourneyTestSuiteAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut include_tests_from_journey_coverage: Option<Option<bool>> = None;
                let mut test_suite_name: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "include_tests_from_journey_coverage" => {
                            include_tests_from_journey_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "test_suite_name" => {
                            test_suite_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DemCreateJourneyTestSuiteAttributes {
                    include_tests_from_journey_coverage,
                    test_suite_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DemCreateJourneyTestSuiteAttributesVisitor)
    }
}
