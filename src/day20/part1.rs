use crate::day20::{GOAL, HOUSE, MULTIPLIER};

pub fn presents(n: u64) -> u64 {
    let mut sum = n * MULTIPLIER;

    for i in 1..n {
        if n % i == 0 {
            sum += i * MULTIPLIER;
        }
    }

    sum
}

pub fn presents_tracking() -> usize {
    let mut houses = vec![10; HOUSE];

    for elf in 2..HOUSE {
        for house in (elf..HOUSE).step_by(elf) {
            let h = houses.get_mut(house - 1).unwrap();
            *h += elf as u64 * MULTIPLIER;
        }
        // println!("{:?}", houses);
        if houses[elf - 1] >= GOAL {
            return elf;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presents() {
        assert_eq!(presents(1), 10);
        assert_eq!(presents(2), 30);
        assert_eq!(presents(3), 40);
        assert_eq!(presents(4), 70);
        assert_eq!(presents(5), 60);
        assert_eq!(presents(6), 120);
        assert_eq!(presents(7), 80);
        assert_eq!(presents(8), 150);
        assert_eq!(presents(9), 130);
    }
}
