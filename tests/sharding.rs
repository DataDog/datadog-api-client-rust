use std::env;

const SHARD_ENV: &str = "DD_TEST_SHARD";
const SHARD_TOTAL_ENV: &str = "DD_TEST_SHARD_TOTAL";
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestShard {
    number: u64,
    total: u64,
}

impl TestShard {
    pub fn from_env() -> Option<Self> {
        match (env::var(SHARD_ENV), env::var(SHARD_TOTAL_ENV)) {
            (Err(env::VarError::NotPresent), Err(env::VarError::NotPresent)) => None,
            (Ok(number), Ok(total)) => Some(
                Self::new(
                    number
                        .parse()
                        .unwrap_or_else(|_| panic!("{SHARD_ENV} must be a positive integer")),
                    total
                        .parse()
                        .unwrap_or_else(|_| panic!("{SHARD_TOTAL_ENV} must be a positive integer")),
                )
                .unwrap_or_else(|error| panic!("{error}")),
            ),
            _ => panic!("{SHARD_ENV} and {SHARD_TOTAL_ENV} must be set together"),
        }
    }

    fn new(number: u64, total: u64) -> Result<Self, String> {
        if total == 0 {
            return Err(format!("{SHARD_TOTAL_ENV} must be greater than zero"));
        }
        if number == 0 || number > total {
            return Err(format!("{SHARD_ENV} must be between 1 and {total}"));
        }
        Ok(Self { number, total })
    }

    pub fn number(self) -> u64 {
        self.number
    }

    pub fn total(self) -> u64 {
        self.total
    }

    pub fn includes(
        self,
        feature_path: &str,
        rule_name: Option<&str>,
        scenario_name: &str,
        scenario_line: usize,
    ) -> bool {
        let mut hash = FNV_OFFSET_BASIS;
        let scenario_line = scenario_line.to_string();
        for component in [
            feature_path,
            rule_name.unwrap_or_default(),
            scenario_name,
            scenario_line.as_str(),
        ] {
            for byte in component.bytes().chain(std::iter::once(0)) {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(FNV_PRIME);
            }
        }
        hash % self.total == self.number - 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn validates_shard_bounds() {
        assert!(super::TestShard::new(0, 4).is_err());
        assert!(super::TestShard::new(5, 4).is_err());
        assert!(super::TestShard::new(1, 0).is_err());
        assert_eq!(super::TestShard::new(1, 4).unwrap().number(), 1);
    }

    #[test]
    fn assigns_each_scenario_to_exactly_one_shard() {
        for scenario in [
            ("features/v1/metrics.feature", None, "list metrics", 10),
            (
                "features/v2/monitors.feature",
                Some("monitor rules"),
                "create monitor",
                42,
            ),
            ("features/v2/users.feature", None, "list users", 18),
        ] {
            let matching = (1..=4)
                .filter(|number| {
                    super::TestShard::new(*number, 4)
                        .unwrap()
                        .includes(scenario.0, scenario.1, scenario.2, scenario.3)
                })
                .count();
            assert_eq!(matching, 1);
        }
    }
}
