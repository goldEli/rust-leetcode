fn main() {
    println!("Hello, world!");
}

mod can_place_flowers_605;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_place_flowers_605() {
        assert_eq!(
            can_place_flowers_605::Solution::can_place_flowers(vec![1,0,0,0,1], 1),
            true
        );
        assert_eq!(
            can_place_flowers_605::Solution::can_place_flowers(vec![1,0,0,0,1], 2),
            false
        );
    }
}