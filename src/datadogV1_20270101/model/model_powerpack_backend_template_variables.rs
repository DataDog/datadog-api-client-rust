// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PowerpackBackendTemplateVariables {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "controlled_by_powerpack")]
    pub controlled_by_powerpack: Vec<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariable>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "controlled_externally")]
    pub controlled_externally: Vec<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariable>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl PowerpackBackendTemplateVariables {
    pub fn new(controlled_by_powerpack: Vec<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariable>, controlled_externally: Vec<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariable>) -> PowerpackBackendTemplateVariables {
        PowerpackBackendTemplateVariables {
            controlled_by_powerpack,
            controlled_externally,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for PowerpackBackendTemplateVariables {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PowerpackBackendTemplateVariablesVisitor;
        impl<'a> Visitor<'a> for PowerpackBackendTemplateVariablesVisitor {
            type Value = PowerpackBackendTemplateVariables;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut controlled_by_powerpack: Option<Vec<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariable>> = None;
                let mut controlled_externally: Option<Vec<crate::datadogV1_20270101::model::PowerpackBackendTemplateVariable>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "controlled_by_powerpack" => {
                            controlled_by_powerpack = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "controlled_externally" => {
                            controlled_externally = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let controlled_by_powerpack = controlled_by_powerpack.ok_or_else(|| M::Error::missing_field("controlled_by_powerpack"))?;
                let controlled_externally = controlled_externally.ok_or_else(|| M::Error::missing_field("controlled_externally"))?;

                let content = PowerpackBackendTemplateVariables {
                    controlled_by_powerpack,
                    controlled_externally,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PowerpackBackendTemplateVariablesVisitor)
    }
}
