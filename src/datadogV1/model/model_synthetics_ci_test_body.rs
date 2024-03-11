// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the synthetics tests to trigger.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsCITestBody {
    /// Individual synthetics test.
    #[serde(rename = "tests")]
    pub tests: Option<Vec<crate::datadogV1::model::SyntheticsCITest>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsCITestBody {
    pub fn new() -> SyntheticsCITestBody {
        SyntheticsCITestBody {
            tests: None,
            _unparsed: false,
        }
    }

    pub fn tests(&mut self, value: Vec<crate::datadogV1::model::SyntheticsCITest>) -> &mut Self {
        self.tests = Some(value);
        self
    }
}

impl Default for SyntheticsCITestBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsCITestBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsCITestBodyVisitor;
        impl<'a> Visitor<'a> for SyntheticsCITestBodyVisitor {
            type Value = SyntheticsCITestBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tests: Option<Vec<crate::datadogV1::model::SyntheticsCITest>> = None;
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

                let content = SyntheticsCITestBody { tests, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsCITestBodyVisitor)
    }
}
