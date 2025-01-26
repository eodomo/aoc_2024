#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Increasing,
    Decreasing,
}
#[derive(Debug)]
pub struct Report {
    pub levels: Vec<i32>,
    direction: Option<Direction>,
    pub safe: bool,
}
impl Report {
    pub fn new() -> Self {
        Report {
            levels: vec![],
            direction: None,
            safe: true,
        }
    }
    pub fn compare_all_values(&mut self) -> bool {
        let window = self.levels.windows(2);
        for slice in window {
            match self.direction {
                None => {
                    let direction = compare_direction(&slice[0], &slice[1]);
                    self.direction = Some(direction);
                    if !compare_two_levels(direction, &slice[0], &slice[1]) {
                        self.safe = false;
                        return false;
                    }
                }
                Some(val) => {
                    if compare_direction(&slice[0], &slice[1]) != val {
                        self.safe = false;
                        return false;
                    }
                    if !compare_two_levels(val, &slice[0], &slice[1]) {
                        self.safe = false;
                        return false;
                    }
                }
            }
        }
        true
    }
}

/// true means safe, false means unsafe
fn compare_two_levels(direction: Direction, num0: &i32, num1: &i32) -> bool {
    if num0 == num1 {
        return false;
    }
    if direction == Direction::Increasing {
        if num0 > num1 || *num1 > num0 + 3 {
            false
        } else {
            true
        }
    } else {
        if num0 < num1 || *num1 < num0 - 3 {
            false
        } else {
            true
        }
    }
}

fn compare_direction(num0: &i32, num1: &i32) -> Direction {
    if num1 > num0 {
        Direction::Increasing
    } else {
        Direction::Decreasing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_direction_increasing() {
        let result = compare_direction(&4, &30);
        assert_eq!(Direction::Increasing, result);
    }

    #[test]
    fn test_compare_direction_decreasing() {
        let result = compare_direction(&30, &4);
        assert_eq!(Direction::Decreasing, result);
    }

    #[test]
    fn test_compare_two_levels_increasing_safe() {
        let direction = Direction::Increasing;
        let num0 = 4;
        let num1 = 7;
        assert_eq!(true, compare_two_levels(direction, &num0, &num1));
    }

    #[test]
    fn test_compare_two_levels_increasing_unsafe_small() {
        let direction = Direction::Increasing;
        let num0 = 4;
        let num1 = 4;
        assert_ne!(true, compare_two_levels(direction, &num0, &num1));
    }

    #[test]
    fn test_compare_two_levels_increasing_unsafe_large() {
        let direction = Direction::Increasing;
        let num0 = 4;
        let num1 = 8;
        assert_ne!(true, compare_two_levels(direction, &num0, &num1));
    }

    #[test]
    fn test_compare_two_levels_decreasing_safe() {
        let direction = Direction::Decreasing;
        let num0 = 7;
        let num1 = 4;
        assert_eq!(true, compare_two_levels(direction, &num0, &num1));
    }

    #[test]
    fn test_compare_two_levels_decreasing_unsafe_small() {
        let direction = Direction::Decreasing;
        let num0 = 7;
        let num1 = 7;
        assert_ne!(true, compare_two_levels(direction, &num0, &num1));
    }

    #[test]
    fn test_compare_two_levels_decreasing_unsafe_large() {
        let direction = Direction::Decreasing;
        let num0 = 7;
        let num1 = 3;
        assert_ne!(true, compare_two_levels(direction, &num0, &num1));
    }

    #[test]
    fn test_compare_all_values_safe_increasing() {
        let mut report = Report::new();
        let levels = vec![1, 3, 5, 8];
        for level in levels {
            report.levels.push(level);
        }
        report.compare_all_values();
        assert_eq!(true, report.safe);
    }

    #[test]
    fn test_compare_all_values_safe_decreasing() {
        let mut report = Report::new();
        let levels = vec![8, 5, 3, 1];
        for level in levels {
            report.levels.push(level);
        }
        report.compare_all_values();
        assert_eq!(true, report.safe);
    }

    #[test]
    fn test_compare_all_values_unsafe_decreasing() {
        let mut report = Report::new();
        let levels = vec![9, 5, 3, 1];
        for level in levels {
            report.levels.push(level);
        }
        report.compare_all_values();
        assert_ne!(true, report.safe);
    }

    #[test]
    fn test_compare_all_values_unsafe_increasing() {
        let mut report = Report::new();
        let levels = vec![1, 5, 7, 9];
        for level in levels {
            report.levels.push(level);
        }
        report.compare_all_values();
        assert_ne!(true, report.safe);
    }
}
