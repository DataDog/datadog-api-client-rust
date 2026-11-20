// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryTableTimestampFormat {
	MMM_DD_YYYY_HH_MM,
	MMM_DD_YYYY_HH_MM_SS,
	MMM_DD_YYYY_H_MM_A,
	MMM_DD_YYYY_H_MM_SS_A,
	MMM_D_HH_MM,
	MMM_D_HH_MM_SS,
	MMM_D_H_MM_A,
	MMM_D_H_MM_SS_A,
	MMM_D__YYYY_HH_MM,
	MMM_D__YYYY_HH_MM_SS,
	MMM_D__YYYY_H_MM_A,
	MMM_D__YYYY_H_MM_SS_A,
	MM_DD_YYYY_HH_MM,
	MM_DD_YYYY_HH_MM_SS,
	MM_DD_YYYY_H_MM_A,
	MM_DD_YYYY_H_MM_SS_A,
	DD_MM_YYYY_HH_MM,
	DD_MM_YYYY_HH_MM_SS,
	DD_MM_YYYY_H_MM_A,
	DD_MM_YYYY_H_MM_SS_A,
	MMMM_D__YYYY_HH_MM,
	MMMM_D__YYYY_HH_MM_SS,
	MMMM_D__YYYY_H_MM_A,
	MMMM_D__YYYY_H_MM_SS_A,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for QueryTableTimestampFormat {
    fn to_string(&self) -> String {
        match self {
            Self::MMM_DD_YYYY_HH_MM => String::from("MMM dd yyyy HH:mm"),
            Self::MMM_DD_YYYY_HH_MM_SS => String::from("MMM dd yyyy HH:mm:ss"),
            Self::MMM_DD_YYYY_H_MM_A => String::from("MMM dd yyyy h:mm a"),
            Self::MMM_DD_YYYY_H_MM_SS_A => String::from("MMM dd yyyy h:mm:ss a"),
            Self::MMM_D_HH_MM => String::from("MMM d HH:mm"),
            Self::MMM_D_HH_MM_SS => String::from("MMM d HH:mm:ss"),
            Self::MMM_D_H_MM_A => String::from("MMM d h:mm a"),
            Self::MMM_D_H_MM_SS_A => String::from("MMM d h:mm:ss a"),
            Self::MMM_D__YYYY_HH_MM => String::from("MMM d, yyyy HH:mm"),
            Self::MMM_D__YYYY_HH_MM_SS => String::from("MMM d, yyyy HH:mm:ss"),
            Self::MMM_D__YYYY_H_MM_A => String::from("MMM d, yyyy h:mm a"),
            Self::MMM_D__YYYY_H_MM_SS_A => String::from("MMM d, yyyy h:mm:ss a"),
            Self::MM_DD_YYYY_HH_MM => String::from("MM/dd/yyyy HH:mm"),
            Self::MM_DD_YYYY_HH_MM_SS => String::from("MM/dd/yyyy HH:mm:ss"),
            Self::MM_DD_YYYY_H_MM_A => String::from("MM/dd/yyyy h:mm a"),
            Self::MM_DD_YYYY_H_MM_SS_A => String::from("MM/dd/yyyy h:mm:ss a"),
            Self::DD_MM_YYYY_HH_MM => String::from("dd/MM/yyyy HH:mm"),
            Self::DD_MM_YYYY_HH_MM_SS => String::from("dd/MM/yyyy HH:mm:ss"),
            Self::DD_MM_YYYY_H_MM_A => String::from("dd/MM/yyyy h:mm a"),
            Self::DD_MM_YYYY_H_MM_SS_A => String::from("dd/MM/yyyy h:mm:ss a"),
            Self::MMMM_D__YYYY_HH_MM => String::from("MMMM d, yyyy HH:mm"),
            Self::MMMM_D__YYYY_HH_MM_SS => String::from("MMMM d, yyyy HH:mm:ss"),
            Self::MMMM_D__YYYY_H_MM_A => String::from("MMMM d, yyyy h:mm a"),
            Self::MMMM_D__YYYY_H_MM_SS_A => String::from("MMMM d, yyyy h:mm:ss a"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for QueryTableTimestampFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for QueryTableTimestampFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"MMM dd yyyy HH:mm" => Self::MMM_DD_YYYY_HH_MM,"MMM dd yyyy HH:mm:ss" => Self::MMM_DD_YYYY_HH_MM_SS,"MMM dd yyyy h:mm a" => Self::MMM_DD_YYYY_H_MM_A,"MMM dd yyyy h:mm:ss a" => Self::MMM_DD_YYYY_H_MM_SS_A,"MMM d HH:mm" => Self::MMM_D_HH_MM,"MMM d HH:mm:ss" => Self::MMM_D_HH_MM_SS,"MMM d h:mm a" => Self::MMM_D_H_MM_A,"MMM d h:mm:ss a" => Self::MMM_D_H_MM_SS_A,"MMM d, yyyy HH:mm" => Self::MMM_D__YYYY_HH_MM,"MMM d, yyyy HH:mm:ss" => Self::MMM_D__YYYY_HH_MM_SS,"MMM d, yyyy h:mm a" => Self::MMM_D__YYYY_H_MM_A,"MMM d, yyyy h:mm:ss a" => Self::MMM_D__YYYY_H_MM_SS_A,"MM/dd/yyyy HH:mm" => Self::MM_DD_YYYY_HH_MM,"MM/dd/yyyy HH:mm:ss" => Self::MM_DD_YYYY_HH_MM_SS,"MM/dd/yyyy h:mm a" => Self::MM_DD_YYYY_H_MM_A,"MM/dd/yyyy h:mm:ss a" => Self::MM_DD_YYYY_H_MM_SS_A,"dd/MM/yyyy HH:mm" => Self::DD_MM_YYYY_HH_MM,"dd/MM/yyyy HH:mm:ss" => Self::DD_MM_YYYY_HH_MM_SS,"dd/MM/yyyy h:mm a" => Self::DD_MM_YYYY_H_MM_A,"dd/MM/yyyy h:mm:ss a" => Self::DD_MM_YYYY_H_MM_SS_A,"MMMM d, yyyy HH:mm" => Self::MMMM_D__YYYY_HH_MM,"MMMM d, yyyy HH:mm:ss" => Self::MMMM_D__YYYY_HH_MM_SS,"MMMM d, yyyy h:mm a" => Self::MMMM_D__YYYY_H_MM_A,"MMMM d, yyyy h:mm:ss a" => Self::MMMM_D__YYYY_H_MM_SS_A,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
