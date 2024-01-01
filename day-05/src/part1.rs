use crate::{Almanac, DestSourceLen};

pub fn get_lowest_location(almanac: &Almanac) -> i64 {
    almanac.seeds.iter().map(|seed| -> i64 {
        seed_number_to_location(seed.clone(), &almanac.mappings)
    }).min().unwrap_or_default()
}

fn seed_number_to_location(seed: i32, mappings: &Vec<Vec<DestSourceLen>>) -> i64 {
    let mut current: i64 = seed.into();
    for mapping in mappings {
        for (dest, source, len) in mapping {
            if ((*source)..(source+len)).contains(&current) {
                current = current + (dest - source);
                break;
            }
        }
    }
    current
}
