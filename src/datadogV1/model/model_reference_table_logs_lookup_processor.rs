// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// **Note**: Reference Tables are in public beta.
/// Use the Lookup Processor to define a mapping between a log attribute
/// and a human readable value saved in a Reference Table.
/// For example, you can use the Lookup Processor to map an internal service ID
/// into a human readable service name. Alternatively, you could also use it to check
/// if the MAC address that just attempted to connect to the production
/// environment belongs to your list of stolen machines.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReferenceTableLogsLookupProcessor {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the Reference Table for the source attribute and their associated target attribute values.
    #[serde(rename = "lookup_enrichment_table")]
    pub lookup_enrichment_table: String,
    /// Name of the processor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Source attribute used to perform the lookup.
    #[serde(rename = "source")]
    pub source: String,
    /// Name of the attribute that contains the corresponding value in the mapping list.
    #[serde(rename = "target")]
    pub target: String,
    /// Type of logs lookup processor.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::LogsLookupProcessorType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReferenceTableLogsLookupProcessor {
    pub fn new(
        lookup_enrichment_table: String,
        source: String,
        target: String,
        type_: crate::datadogV1::model::LogsLookupProcessorType,
    ) -> ReferenceTableLogsLookupProcessor {
        ReferenceTableLogsLookupProcessor {
            is_enabled: None,
            lookup_enrichment_table,
            name: None,
            source,
            target,
            type_,
            _unparsed: false,
        }
    }

    pub fn is_enabled(&mut self, value: bool) -> &mut Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ReferenceTableLogsLookupProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReferenceTableLogsLookupProcessorVisitor;
        impl<'a> Visitor<'a> for ReferenceTableLogsLookupProcessorVisitor {
            type Value = ReferenceTableLogsLookupProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_enabled: Option<bool> = None;
                let mut lookup_enrichment_table: Option<String> = None;
                let mut name: Option<String> = None;
                let mut source: Option<String> = None;
                let mut target: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::LogsLookupProcessorType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "lookup_enrichment_table" => {
                            lookup_enrichment_table =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::LogsLookupProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let lookup_enrichment_table = lookup_enrichment_table
                    .ok_or_else(|| M::Error::missing_field("lookup_enrichment_table"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let target = target.ok_or_else(|| M::Error::missing_field("target"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ReferenceTableLogsLookupProcessor {
                    is_enabled,
                    lookup_enrichment_table,
                    name,
                    source,
                    target,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReferenceTableLogsLookupProcessorVisitor)
    }
}
