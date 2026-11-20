// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for a URL that should have distributed tracing enabled.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMSdkConfigTracingUrlConfig {
    /// A match option used for URL or origin pattern matching.
    #[serde(rename = "match")]
    pub match_: crate::datadogV2::model::RUMSdkConfigMatchOption,
    /// The list of trace propagator types to use for this URL.
    #[serde(rename = "propagator_types")]
    pub propagator_types: Vec<crate::datadogV2::model::RUMSdkConfigTracingUrlPropagatorType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMSdkConfigTracingUrlConfig {
    pub fn new(
        match_: crate::datadogV2::model::RUMSdkConfigMatchOption,
        propagator_types: Vec<crate::datadogV2::model::RUMSdkConfigTracingUrlPropagatorType>,
    ) -> RUMSdkConfigTracingUrlConfig {
        RUMSdkConfigTracingUrlConfig {
            match_,
            propagator_types,
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

impl<'de> Deserialize<'de> for RUMSdkConfigTracingUrlConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMSdkConfigTracingUrlConfigVisitor;
        impl<'a> Visitor<'a> for RUMSdkConfigTracingUrlConfigVisitor {
            type Value = RUMSdkConfigTracingUrlConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut match_: Option<crate::datadogV2::model::RUMSdkConfigMatchOption> = None;
                let mut propagator_types: Option<
                    Vec<crate::datadogV2::model::RUMSdkConfigTracingUrlPropagatorType>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "match" => {
                            match_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "propagator_types" => {
                            propagator_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let match_ = match_.ok_or_else(|| M::Error::missing_field("match_"))?;
                let propagator_types =
                    propagator_types.ok_or_else(|| M::Error::missing_field("propagator_types"))?;

                let content = RUMSdkConfigTracingUrlConfig {
                    match_,
                    propagator_types,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMSdkConfigTracingUrlConfigVisitor)
    }
}
