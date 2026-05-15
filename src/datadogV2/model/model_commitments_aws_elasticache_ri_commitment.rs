// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS ElastiCache Reserved Instance commitment details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsAwsElasticacheRICommitment {
    /// The cache engine type of the Reserved Instance.
    #[serde(rename = "cache_engine")]
    pub cache_engine: String,
    /// The unique identifier of the Reserved Instance.
    #[serde(rename = "commitment_id")]
    pub commitment_id: String,
    /// The expiration date of the commitment.
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<String>,
    /// The ElastiCache instance type.
    #[serde(rename = "instance_type")]
    pub instance_type: String,
    /// The number of Normalized Capacity Units.
    #[serde(rename = "number_of_nfus")]
    pub number_of_nfus: Option<f64>,
    /// The number of reserved instances.
    #[serde(rename = "number_of_reservations")]
    pub number_of_reservations: Option<f64>,
    /// The payment option for the Reserved Instance.
    #[serde(rename = "purchase_option")]
    pub purchase_option: String,
    /// The AWS region of the Reserved Instance.
    #[serde(rename = "region")]
    pub region: String,
    /// The start date of the commitment.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// The term length in years.
    #[serde(rename = "term_length")]
    pub term_length: Option<f64>,
    /// The utilization percentage of the commitment.
    #[serde(rename = "utilization")]
    pub utilization: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsAwsElasticacheRICommitment {
    pub fn new(
        cache_engine: String,
        commitment_id: String,
        instance_type: String,
        purchase_option: String,
        region: String,
    ) -> CommitmentsAwsElasticacheRICommitment {
        CommitmentsAwsElasticacheRICommitment {
            cache_engine,
            commitment_id,
            expiration_date: None,
            instance_type,
            number_of_nfus: None,
            number_of_reservations: None,
            purchase_option,
            region,
            start_date: None,
            term_length: None,
            utilization: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn expiration_date(mut self, value: String) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn number_of_nfus(mut self, value: f64) -> Self {
        self.number_of_nfus = Some(value);
        self
    }

    pub fn number_of_reservations(mut self, value: f64) -> Self {
        self.number_of_reservations = Some(value);
        self
    }

    pub fn start_date(mut self, value: String) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn term_length(mut self, value: f64) -> Self {
        self.term_length = Some(value);
        self
    }

    pub fn utilization(mut self, value: f64) -> Self {
        self.utilization = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsAwsElasticacheRICommitment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsAwsElasticacheRICommitmentVisitor;
        impl<'a> Visitor<'a> for CommitmentsAwsElasticacheRICommitmentVisitor {
            type Value = CommitmentsAwsElasticacheRICommitment;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cache_engine: Option<String> = None;
                let mut commitment_id: Option<String> = None;
                let mut expiration_date: Option<String> = None;
                let mut instance_type: Option<String> = None;
                let mut number_of_nfus: Option<f64> = None;
                let mut number_of_reservations: Option<f64> = None;
                let mut purchase_option: Option<String> = None;
                let mut region: Option<String> = None;
                let mut start_date: Option<String> = None;
                let mut term_length: Option<f64> = None;
                let mut utilization: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cache_engine" => {
                            cache_engine =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "commitment_id" => {
                            commitment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration_date" => {
                            if v.is_null() {
                                continue;
                            }
                            expiration_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "instance_type" => {
                            instance_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number_of_nfus" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            number_of_nfus =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "number_of_reservations" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            number_of_reservations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "purchase_option" => {
                            purchase_option =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "term_length" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            term_length =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "utilization" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            utilization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cache_engine =
                    cache_engine.ok_or_else(|| M::Error::missing_field("cache_engine"))?;
                let commitment_id =
                    commitment_id.ok_or_else(|| M::Error::missing_field("commitment_id"))?;
                let instance_type =
                    instance_type.ok_or_else(|| M::Error::missing_field("instance_type"))?;
                let purchase_option =
                    purchase_option.ok_or_else(|| M::Error::missing_field("purchase_option"))?;
                let region = region.ok_or_else(|| M::Error::missing_field("region"))?;

                let content = CommitmentsAwsElasticacheRICommitment {
                    cache_engine,
                    commitment_id,
                    expiration_date,
                    instance_type,
                    number_of_nfus,
                    number_of_reservations,
                    purchase_option,
                    region,
                    start_date,
                    term_length,
                    utilization,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsAwsElasticacheRICommitmentVisitor)
    }
}
