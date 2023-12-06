use crate::Solution;

static RACES: [Race; 4] = [
    Race {
        time: 50,
        record: 242,
    },
    Race {
        time: 74,
        record: 1017,
    },
    Race {
        time: 86,
        record: 1691,
    },
    Race {
        time: 85,
        record: 1252,
    },
];

static RACE: Race = Race {
    time: 50748685,
    record: 242101716911252,
};

pub struct Day6;

impl Solution for Day6 {
    fn day(&self) -> u8 {
        6
    }

    fn part_one(&self) -> String {
        format!(
            "Product of all ways to win races: {}",
            ways_to_win_product(&RACES),
        )
    }

    fn part_two(&self) -> String {
        format!("Ways to win the race: {}", RACE.ways_to_win_count())
    }
}

fn ways_to_win_product(races: &[Race]) -> u64 {
    races.iter().map(Race::ways_to_win_count).product()
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn hold(&self, hold_time: u64) -> u64 {
        hold_time * (self.time - hold_time)
    }

    fn min_hold_time(&self) -> Option<u64> {
        (1..self.time).find(|h| self.hold(*h) > self.record)
    }

    fn max_hold_time(&self) -> Option<u64> {
        (1..self.time).rev().find(|h| self.hold(*h) > self.record)
    }

    fn ways_to_win_count(&self) -> u64 {
        if let Some((min, max)) = self.min_hold_time().zip(self.max_hold_time()) {
            max - min + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_RACES: [Race; 3] = [
        Race { time: 7, record: 9 },
        Race {
            time: 15,
            record: 40,
        },
        Race {
            time: 30,
            record: 200,
        },
    ];

    static EXAMPLE_RACE: Race = Race {
        time: 71530,
        record: 940200,
    };

    #[test]
    fn min_hold_time_should_return_2_for_example_1() {
        assert_eq!(EXAMPLE_RACES[0].min_hold_time(), Some(2));
    }

    #[test]
    fn max_hold_time_should_return_5_for_example_1() {
        assert_eq!(EXAMPLE_RACES[0].max_hold_time(), Some(5));
    }

    #[test]
    fn ways_to_win_count_should_return_4_for_example_1() {
        assert_eq!(EXAMPLE_RACES[0].ways_to_win_count(), 4);
    }

    #[test]
    fn ways_to_win_count_should_return_8_for_example_2() {
        assert_eq!(EXAMPLE_RACES[1].ways_to_win_count(), 8);
    }

    #[test]
    fn ways_to_win_count_should_return_9_for_example_3() {
        assert_eq!(EXAMPLE_RACES[2].ways_to_win_count(), 9);
    }

    #[test]
    fn ways_to_win_product_should_return_288_for_example() {
        assert_eq!(ways_to_win_product(&EXAMPLE_RACES), 288);
    }

    #[test]
    fn ways_to_win_count_should_return_71503_for_example() {
        assert_eq!(EXAMPLE_RACE.ways_to_win_count(), 71503);
    }
}
