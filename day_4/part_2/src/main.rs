use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[macro_use]
use phf::{phf_map, phf_set};
use regex::Regex;


static REQUIRED_KEYS: phf::Set<&'static str> = phf_set! {
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"
};


type Callable = fn(&str) -> bool;


static VAL_CHECK_FUNCS: phf::Map<&'static str, Callable> = phf_map! {
    "byr" => is_byr_valid,
    "iyr" => is_iyr_valid,
    "eyr" => is_eyr_valid,
    "hgt" => is_hgt_valid,
    "hcl" => is_hcl_valid,
    "ecl" => is_ecl_valid,
    "pid" => is_pid_valid
};


static VALID_EYE_COLORS: phf::Set<&'static str> = phf_set! {
    "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let n_valid_passwords = count_valid_passports(f);
    println!("found {} valid passports", n_valid_passwords);
}


fn count_valid_passports<'a>(read_buffer: BufReader<File>) -> i32 {
    let passports = parse_passports(read_buffer);
    passports.iter().filter(|p| p.is_valid()).count() as i32
}


struct PassportKeys {
    key_vals: HashMap<String, String>
}


impl PassportKeys {
    fn keys(&self) -> HashSet<String> {
        self.key_vals.keys().cloned().collect()
    }    
    
    fn contains_necessary_keys(&self) -> bool {        
        for key in REQUIRED_KEYS.iter() {
            if !self.keys().contains(&key[..]) {
                return false
            }
        }
        true
    }

    fn is_valid(&self) -> bool {
        if !self.contains_necessary_keys() {
            return false
        }
        for key in REQUIRED_KEYS.iter() {
            if !VAL_CHECK_FUNCS[key](&self.key_vals.get(&key[..]).unwrap()) {
                return false
            }
        }
        true
    }
}


fn parse_passports(read_buffer: BufReader<File>) -> Vec<PassportKeys> {
    let mut passports: Vec<PassportKeys> = Vec::new();
    let mut found_key_vals: HashMap<String, String> = HashMap::new();

    for line in read_buffer.lines() {
        let line = line.expect("could not read line!");
        if line != "" {
            found_key_vals = merge_maps(found_key_vals, parse_keys(line)); 
        } else {
            passports.push(
                PassportKeys{
                    key_vals: found_key_vals
                }
            );
            found_key_vals = HashMap::new();
        }
    }
    if found_key_vals.len() > 0 {
        passports.push(
            PassportKeys{
                key_vals: found_key_vals
            }
        );
    }

    passports
}


fn parse_keys(line: String) -> HashMap<String, String> {
    let mut key_vals: HashMap<String, String> = HashMap::new();

    let re: Regex = Regex::new(
        r"([^ ]*):([^ ]*)"
    ).unwrap();
    let captures = re.captures_iter(&line);
    for capture in captures {
        key_vals.insert(
            capture.get(1).unwrap().as_str().to_string(),
            capture.get(2).unwrap().as_str().to_string()
        );
    }
    key_vals
}


fn merge_maps(map_1: HashMap<String, String>, map_2: HashMap<String, String>) -> HashMap<String, String> {
    let mut new_map: HashMap<String, String> = HashMap::new();
    for (key, value) in map_1.into_iter() {
        new_map.insert(key, value);
    }
    for (key, value) in map_2.into_iter() {
        new_map.insert(key, value);
    }
    new_map
}


fn is_four_digits(val: &str) -> bool {
    let re: Regex = Regex::new(
        r"^[0-9]{4}$"
    ).unwrap();
    !(re.captures(&val).iter().len() == 0)
}

fn is_in_inclusive_range(val: &i32, min: i32, max: i32) -> bool {
     ((min <= *val) && (*val <= max))
}


fn is_byr_valid(byr: &str) -> bool {
    if !is_four_digits(byr) {
        return false
    }

    let year: i32 = byr.parse::<i32>().unwrap();
    if !is_in_inclusive_range(&year, 1920, 2002) {
        return false
    }

    true
}


fn is_iyr_valid(iyr: &str) -> bool {
        if !is_four_digits(iyr) {
        return false
    }

    let year: i32 = iyr.parse::<i32>().unwrap();
    if !is_in_inclusive_range(&year, 2010, 2020) {
        return false
    }

    true
}


fn is_eyr_valid(eyr: &str) -> bool {
        if !is_four_digits(eyr) {
        return false
    }

    let year: i32 = eyr.parse::<i32>().unwrap();
    if !is_in_inclusive_range(&year, 2020, 2030) {
        return false
    }

    true
}


fn is_hgt_valid(hgt: &str) -> bool {
    let re: Regex = Regex::new(
        r"^([0-9]*)([incm]{2})$"
    ).unwrap();
    if re.captures(&hgt).iter().len() == 0 {
        return false
    }

    let (height, unit) = re.captures(hgt).and_then(|cap| {
        Some((
            cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            cap.get(2).unwrap().as_str()
        ))
    }).unwrap();
    if (unit == "cm") && !is_in_inclusive_range(&height, 150, 193) {
        return false
    }
    if (unit == "in") && !is_in_inclusive_range(&height, 59, 76) {
        return false
    }
    true
}

fn is_hcl_valid(hcl: &str) -> bool {
    let re: Regex = Regex::new(
        r"^\#[0-9a-f]{6}$"
    ).unwrap();
    !(re.captures(&hcl).iter().len() == 0)
}


fn is_ecl_valid(ecl: &str) -> bool {
    VALID_EYE_COLORS.contains(ecl)
}


fn is_pid_valid(pid: &str) -> bool {
    let re: Regex = Regex::new(
        r"^[0-9]{9}$"
    ).unwrap();
    !(re.captures(&pid).iter().len() == 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_byr_valid() {
        assert_eq!(is_byr_valid("192"), false);
        assert_eq!(is_byr_valid("1920"), true);
        assert_eq!(is_byr_valid("19210"), false);
        assert_eq!(is_byr_valid("1919"), false);
        assert_eq!(is_byr_valid("2003"), false);
    }

    #[test]
    fn test_is_iyr_valid() {
        assert_eq!(is_iyr_valid("192"), false);
        assert_eq!(is_iyr_valid("2010"), true);
        assert_eq!(is_iyr_valid("19210"), false);
        assert_eq!(is_iyr_valid("2009"), false);
        assert_eq!(is_iyr_valid("2021"), false);
    }
    #[test]
    fn test_is_hgt_valid() {
        assert_eq!(is_hgt_valid("150cm"), true);
        assert_eq!(is_hgt_valid("149cm"), false);
        assert_eq!(is_hgt_valid("59in"), true);
        assert_eq!(is_hgt_valid("58in"), false);
    }

    #[test]
    fn test_is_hcl_valid() {
        assert_eq!(is_hcl_valid("#abcde0"), true);
        assert_eq!(is_hcl_valid("abcde1"), false);
        assert_eq!(is_hcl_valid("#zabcde"), false);
    }

    #[test]
    fn test_is_ecl_valid() {
        assert_eq!(is_ecl_valid("amb"), true);
        assert_eq!(is_ecl_valid("foo"), false);
    }

}