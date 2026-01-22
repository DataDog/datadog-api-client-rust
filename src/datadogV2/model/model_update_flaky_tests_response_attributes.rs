// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the update flaky test state response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateFlakyTestsResponseAttributes {
    /// `True` if any errors occurred during the update operations. `False` if all tests succeeded to be updated.
    #[serde(rename = "has_errors")]
    pub has_errors: bool,
    /// Results of the update operation for each test.
    #[serde(rename = "results")]
    pub results: Vec<crate::datadogV2::model::UpdateFlakyTestsResponseResult>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateFlakyTestsResponseAttributes {
    pub fn new(
        has_errors: bool,
        results: Vec<crate::datadogV2::model::UpdateFlakyTestsResponseResult>,
    ) -> UpdateFlakyTestsResponseAttributes {
        UpdateFlakyTestsResponseAttributes {
            has_errors,
            results,
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

impl<'de> Deserialize<'de> for UpdateFlakyTestsResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateFlakyTestsResponseAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateFlakyTestsResponseAttributesVisitor {
            type Value = UpdateFlakyTestsResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_errors: Option<bool> = None;
                let mut results: Option<
                    Vec<crate::datadogV2::model::UpdateFlakyTestsResponseResult>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_errors" => {
                            has_errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "results" => {
                            results = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let has_errors = has_errors.ok_or_else(|| M::Error::missing_field("has_errors"))?;
                let results = results.ok_or_else(|| M::Error::missing_field("results"))?;

                let content = UpdateFlakyTestsResponseAttributes {
                    has_errors,
                    results,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateFlakyTestsResponseAttributesVisitor)
    }
}
