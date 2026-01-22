// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of what tests to update and their new attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateFlakyTestsRequestTest {
    /// The ID of the flaky test. This is the same ID returned by the Search flaky tests endpoint and corresponds to the test_fingerprint_fqn field in test run events.
    #[serde(rename = "id")]
    pub id: String,
    /// The new state to set for the flaky test.
    #[serde(rename = "new_state")]
    pub new_state: crate::datadogV2::model::UpdateFlakyTestsRequestTestNewState,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateFlakyTestsRequestTest {
    pub fn new(
        id: String,
        new_state: crate::datadogV2::model::UpdateFlakyTestsRequestTestNewState,
    ) -> UpdateFlakyTestsRequestTest {
        UpdateFlakyTestsRequestTest {
            id,
            new_state,
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

impl<'de> Deserialize<'de> for UpdateFlakyTestsRequestTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateFlakyTestsRequestTestVisitor;
        impl<'a> Visitor<'a> for UpdateFlakyTestsRequestTestVisitor {
            type Value = UpdateFlakyTestsRequestTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut new_state: Option<
                    crate::datadogV2::model::UpdateFlakyTestsRequestTestNewState,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_state" => {
                            new_state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _new_state) = new_state {
                                match _new_state {
                                    crate::datadogV2::model::UpdateFlakyTestsRequestTestNewState::UnparsedObject(_new_state) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let new_state = new_state.ok_or_else(|| M::Error::missing_field("new_state"))?;

                let content = UpdateFlakyTestsRequestTest {
                    id,
                    new_state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateFlakyTestsRequestTestVisitor)
    }
}
