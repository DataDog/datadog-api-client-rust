// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object for deleting Synthetic tests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDeleteTestsResponse {
    /// Array of objects containing a deleted Synthetic test ID with
    /// the associated deletion timestamp.
    #[serde(rename = "deleted_tests")]
    pub deleted_tests: Option<Vec<crate::datadogV1::model::SyntheticsDeletedTest>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDeleteTestsResponse {
    pub fn new() -> SyntheticsDeleteTestsResponse {
        SyntheticsDeleteTestsResponse {
            deleted_tests: None,
            _unparsed: false,
        }
    }

    pub fn deleted_tests(
        mut self,
        value: Vec<crate::datadogV1::model::SyntheticsDeletedTest>,
    ) -> Self {
        self.deleted_tests = Some(value);
        self
    }
}

impl Default for SyntheticsDeleteTestsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsDeleteTestsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDeleteTestsResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsDeleteTestsResponseVisitor {
            type Value = SyntheticsDeleteTestsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_tests: Option<Vec<crate::datadogV1::model::SyntheticsDeletedTest>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_tests" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_tests =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsDeleteTestsResponse {
                    deleted_tests,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDeleteTestsResponseVisitor)
    }
}
