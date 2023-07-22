/*
     通过assert_eq! 和 assert_ne! 这两个宏分别比较两个值是相等还是不相等
 */
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_three(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_three() {
        assert_ne!(4, add_three(2));
    }
}
