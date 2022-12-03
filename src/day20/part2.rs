use crate::day20::{GOAL, HOUSE};

const MULTIPLIER: u64 = 11;

pub fn presents_tracking() -> usize {
    let mut houses = vec![10; HOUSE];

    for elf in 2..HOUSE {
        for house in (elf..HOUSE).step_by(elf).take(50) {
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
