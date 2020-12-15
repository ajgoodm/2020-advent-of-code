use std::collections::HashMap;

const INPUT: [i64; 6] = [1i64, 20, 11, 6, 12, 0];
const NTH_NUM: i64 = 30_000_000;

fn main() {
    let mut history: HashMap<i64, i64> = HashMap::new();
    let mut n_turn: i64 = 0;
    for i in INPUT.iter() {
        n_turn += 1;
        history.insert(*i, n_turn);
    }

    let mut next: i64 = 0;
    while n_turn < NTH_NUM - 1 {
        n_turn += 1;
        if history.contains_key(&next) {
            let tmp: i64 = n_turn - history[&next];
            history.insert(next, n_turn);
            next = tmp;
        } else {
            history.insert(next, n_turn);
            next = 0;
        }
    }
    println!("The {}th number is: {}", NTH_NUM, next);
}


