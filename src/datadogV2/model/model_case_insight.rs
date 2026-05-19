// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A reference to an external Datadog resource that provides investigative context for a case, such as a security signal, monitor alert, error tracking issue, or incident.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseInsight {
    /// The URL path or deep link to the insight resource within Datadog (for example, `/monitors/12345?q=total`).
    #[serde(rename = "ref")]
    pub ref_: String,
    /// The unique identifier of the referenced Datadog resource (for example, a monitor ID, incident ID, or signal ID).
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The type of Datadog resource linked to the case as contextual evidence. Each type corresponds to a different Datadog product signal (for example, a security finding, a monitor alert, or an incident).
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CaseInsightType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseInsight {
    pub fn new(
        ref_: String,
        resource_id: String,
        type_: crate::datadogV2::model::CaseInsightType,
    ) -> CaseInsight {
        CaseInsight {
            ref_,
            resource_id,
            type_,
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

impl<'de> Deserialize<'de> for CaseInsight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseInsightVisitor;
        impl<'a> Visitor<'a> for CaseInsightVisitor {
            type Value = CaseInsight;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ref_: Option<String> = None;
                let mut resource_id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CaseInsightType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ref" => {
                            ref_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CaseInsightType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let ref_ = ref_.ok_or_else(|| M::Error::missing_field("ref_"))?;
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CaseInsight {
                    ref_,
                    resource_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseInsightVisitor)
    }
}
