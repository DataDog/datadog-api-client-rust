// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Live call information for a Page triggered by a live call route.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPageLiveCall {
    /// Information about a live call that has ended.
    #[serde(rename = "call_over")]
    pub call_over: Option<crate::datadogV2::model::OnCallPageLiveCallOver>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPageLiveCall {
    pub fn new() -> OnCallPageLiveCall {
        OnCallPageLiveCall {
            call_over: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn call_over(mut self, value: crate::datadogV2::model::OnCallPageLiveCallOver) -> Self {
        self.call_over = Some(value);
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

impl Default for OnCallPageLiveCall {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OnCallPageLiveCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPageLiveCallVisitor;
        impl<'a> Visitor<'a> for OnCallPageLiveCallVisitor {
            type Value = OnCallPageLiveCall;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut call_over: Option<crate::datadogV2::model::OnCallPageLiveCallOver> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "call_over" => {
                            if v.is_null() {
                                continue;
                            }
                            call_over = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OnCallPageLiveCall {
                    call_over,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPageLiveCallVisitor)
    }
}
