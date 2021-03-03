pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum2plus2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sum2plus2Err() {
        assert_eq!(2 + 2, 5, "4 is not equal to 5");
    }

    #[test]
    #[should_panic(expected = "sample error")]
    fn shouldPanic() {
        panic!("sample error");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
