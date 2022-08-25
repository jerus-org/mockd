use simplerand::{rand_range, Randomable};
use std::clone::Clone;

pub(crate) const HASHTAG: &str = "#";
pub(crate) const QUESTIONMARK: &str = "?";

/// Return a random value from within an array of data.
///
pub(crate) fn random_data<T: Clone>(d: &[T]) -> T {
    let n = rand_range(0, d.len() as i64);
    d[n as usize].clone()
}

/// Return a valid random index to the data within an array
///
pub(crate) fn random_data_index<T>(d: &[T]) -> usize {
    rand_range(0, d.len() as i64) as usize
}

/// Return a random value within a range
///
pub(crate) fn random<T: Randomable>(min: T, max: T) -> T {
    rand_range::<T>(min, max)
}

/// Replace hashtags (octothorps) in a string with random numbers
///
pub(crate) fn replace_with_numbers(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == HASHTAG {
                let i = random::<i64>(0, 9);
                return i.to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

/// Replace question marks in string with valid hex letter
///
pub(crate) fn replace_with_letter_hex(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let letters: [&'static str; 6] = ["a", "b", "c", "d", "e", "f"];

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == QUESTIONMARK {
                let i = random::<usize>(0, 5);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

/// replace question marks in string with random letter
///
pub(crate) fn replace_with_letter(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let letters: [&'static str; 26] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    let res: Vec<String> = s
        .split("")
        .map(|s| {
            if s == QUESTIONMARK {
                let i = random::<usize>(0, letters.len() - 1);
                return letters[i].to_string();
            }
            s.to_string()
        })
        .collect();

    res.join("")
}

/// Return a random character from within the provided string
///
pub(crate) fn random_char_from_string(s: &[u8]) -> char {
    let end_boundary = s.len() - 1;
    let n = rand_range(0, end_boundary as i64);
    s[n as usize] as char
}

#[cfg(test)]
mod tests {
    use crate::misc;
    use std::{thread, time};

    #[allow(dead_code)]
    pub static STREET_NAME: &[&str] = &[
        "Alley",
        "Avenue",
        "Branch",
        "Bridge",
        "Brook",
        "Brooks",
        "Burg",
        "Burgs",
        "Bypass",
        "Camp",
        "Canyon",
        "Cape",
        "Causeway",
        "Center",
        "Centers",
        "Circle",
        "Circles",
        "Cliff",
        "Cliffs",
        "Club",
        "Common",
        "Corner",
        "Corners",
        "Course",
        "Court",
        "Courts",
        "Cove",
        "Coves",
        "Creek",
        "Crescent",
        "Crest",
        "Crossing",
        "Crossroad",
        "Curve",
        "Dale",
        "Dam",
        "Divide",
        "Drive",
        "Drives",
        "Estate",
        "Estates",
        "Expressway",
        "Extension",
        "Extensions",
        "Fall",
        "Falls",
        "Ferry",
        "Field",
        "Fields",
        "Flat",
        "Flats",
        "Ford",
        "Fords",
        "Forest",
        "Forge",
        "Forges",
        "Fork",
        "Forks",
        "Fort",
        "Freeway",
        "Garden",
        "Gardens",
        "Gateway",
        "Glen",
        "Glens",
        "Green",
        "Greens",
        "Grove",
        "Groves",
        "Harbor",
        "Harbors",
        "Haven",
        "Heights",
        "Highway",
        "Hill",
        "Hills",
        "Hollow",
        "Inlet",
        "Island",
        "Islands",
        "Isle",
        "Junction",
        "Junctions",
        "Key",
        "Keys",
        "Knoll",
        "Knolls",
        "Lake",
        "Lakes",
        "Land",
        "Landing",
        "Lane",
        "Light",
        "Lights",
        "Loaf",
        "Lock",
        "Locks",
        "Lodge",
        "Loop",
        "Mall",
        "Manor",
        "Manors",
        "Meadow",
        "Meadows",
        "Mews",
        "Mill",
        "Mills",
        "Mission",
        "Motorway",
        "Mount",
        "Mountain",
        "Mountains",
        "Neck",
        "Orchard",
        "Oval",
        "Overpass",
        "Park",
        "Parks",
        "Parkway",
        "Parkways",
        "Pass",
        "Passage",
        "Path",
        "Pike",
        "Pine",
        "Pines",
        "Place",
        "Plain",
        "Plains",
        "Plaza",
        "Point",
        "Points",
        "Port",
        "Ports",
        "Prairie",
        "Radial",
        "Ramp",
        "Ranch",
        "Rapid",
        "Rapids",
        "Rest",
        "Ridge",
        "Ridges",
        "River",
        "Road",
        "Roads",
        "Route",
        "Row",
        "Rue",
        "Run",
        "Shoal",
        "Shoals",
        "Shore",
        "Shores",
        "Skyway",
        "Spring",
        "Springs",
        "Spur",
        "Spurs",
        "Square",
        "Squares",
        "Station",
        "Stravenue",
        "Stream",
        "Street",
        "Streets",
        "Summit",
        "Terrace",
        "Throughway",
        "Trace",
        "Track",
        "Trafficway",
        "Trail",
        "Tunnel",
        "Turnpike",
        "Underpass",
        "Union",
        "Unions",
        "Valley",
        "Valleys",
        "Via",
        "Viaduct",
        "View",
        "Views",
        "Village",
        "Villages",
        "Ville",
        "Vista",
        "Walk",
        "Walks",
        "Wall",
        "Way",
        "Ways",
        "Well",
        "Wells",
    ];

    #[test]
    fn random_data_test() {
        let mut street1 = misc::random_data(STREET_NAME);
        println!("{}", street1);

        street1 = misc::random_data(STREET_NAME);
        println!("{}", street1);

        street1 = misc::random_data(STREET_NAME);
        println!("{}", street1);
    }

    #[test]
    fn random_data_str() {
        let street1 = misc::random_data(STREET_NAME);
        thread::sleep(time:;duration::from_secs(1));
        let street2 = misc::random_data(STREET_NAME);
        assert_ne!(street1, street2);
    }

    #[test]
    fn replace_with_numbers() {
        let data1 = "a####".to_string();
        let data2 = misc::replace_with_numbers("a####".to_string());
        assert_ne!(data1, data2);
    }
}
