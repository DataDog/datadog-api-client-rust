// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AWSNamespace {
    #[serde(rename = "elb")]
	ELB,
    #[serde(rename = "application_elb")]
	APPLICATION_ELB,
    #[serde(rename = "sqs")]
	SQS,
    #[serde(rename = "rds")]
	RDS,
    #[serde(rename = "custom")]
	CUSTOM,
    #[serde(rename = "network_elb")]
	NETWORK_ELB,
    #[serde(rename = "lambda")]
	LAMBDA,
}

impl ToString for AWSNamespace {
    fn to_string(&self) -> String {
        match self {
            Self::ELB => String::from("elb"),
            Self::APPLICATION_ELB => String::from("application_elb"),
            Self::SQS => String::from("sqs"),
            Self::RDS => String::from("rds"),
            Self::CUSTOM => String::from("custom"),
            Self::NETWORK_ELB => String::from("network_elb"),
            Self::LAMBDA => String::from("lambda"),
        }
    }
}

impl Default for AWSNamespace {
    fn default() -> AWSNamespace {
        Self::ELB
    }
}
