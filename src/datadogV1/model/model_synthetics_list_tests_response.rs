// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing an array of Synthetic tests configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsListTestsResponse {
    /// Array of Synthetic tests configuration.
    #[serde(rename = "tests")]
    pub tests: Option<Vec<crate::datadogV1::model::SyntheticsTestDetails>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsListTestsResponse {
    pub fn new() -> SyntheticsListTestsResponse {
        SyntheticsListTestsResponse {
            tests: None,
            _unparsed: false,
        }
    }

    pub fn tests(mut self, value: Vec<crate::datadogV1::model::SyntheticsTestDetails>) -> Self {
        self.tests = Some(value);
        self
    }
}

impl Default for SyntheticsListTestsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsListTestsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsListTestsResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsListTestsResponseVisitor {
            type Value = SyntheticsListTestsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tests: Option<Vec<crate::datadogV1::model::SyntheticsTestDetails>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tests" => {
                            if v.is_null() {
                                continue;
                            }
                            tests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsListTestsResponse { tests, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsListTestsResponseVisitor)
    }
}
