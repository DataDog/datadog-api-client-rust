// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The status page associated with the email access policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPageEmailAccessPolicyDataRelationshipsStatusPage {
    /// The data object identifying the status page associated with the email access policy.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsStatusPageData,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPageEmailAccessPolicyDataRelationshipsStatusPage {
    pub fn new(
        data: crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsStatusPageData,
    ) -> StatusPageEmailAccessPolicyDataRelationshipsStatusPage {
        StatusPageEmailAccessPolicyDataRelationshipsStatusPage {
            data,
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

impl<'de> Deserialize<'de> for StatusPageEmailAccessPolicyDataRelationshipsStatusPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageEmailAccessPolicyDataRelationshipsStatusPageVisitor;
        impl<'a> Visitor<'a> for StatusPageEmailAccessPolicyDataRelationshipsStatusPageVisitor {
            type Value = StatusPageEmailAccessPolicyDataRelationshipsStatusPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::StatusPageEmailAccessPolicyDataRelationshipsStatusPageData> = None;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = StatusPageEmailAccessPolicyDataRelationshipsStatusPage {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageEmailAccessPolicyDataRelationshipsStatusPageVisitor)
    }
}
