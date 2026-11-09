// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response object for a list of status page email access policies.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPageEmailAccessPolicyArrayResponse {
    /// A list of status page email access policy data objects.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::StatusPageEmailAccessPolicyData>,
    /// Resources related to the email access policies, when requested using the `include` query parameter.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::StatusPageEmailAccessPolicyIncluded>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPageEmailAccessPolicyArrayResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::StatusPageEmailAccessPolicyData>,
    ) -> StatusPageEmailAccessPolicyArrayResponse {
        StatusPageEmailAccessPolicyArrayResponse {
            data,
            included: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::StatusPageEmailAccessPolicyIncluded>,
    ) -> Self {
        self.included = Some(value);
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

impl<'de> Deserialize<'de> for StatusPageEmailAccessPolicyArrayResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageEmailAccessPolicyArrayResponseVisitor;
        impl<'a> Visitor<'a> for StatusPageEmailAccessPolicyArrayResponseVisitor {
            type Value = StatusPageEmailAccessPolicyArrayResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Vec<crate::datadogV2::model::StatusPageEmailAccessPolicyData>,
                > = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::StatusPageEmailAccessPolicyIncluded>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = StatusPageEmailAccessPolicyArrayResponse {
                    data,
                    included,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageEmailAccessPolicyArrayResponseVisitor)
    }
}
