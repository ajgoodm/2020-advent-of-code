use std::collections::HashSet;

// const INPUT: &str = "389125467";
const INPUT: &str = "394618527";
const SIZE: usize = INPUT.len();
const MIN_VAL: usize = 1;
const MAX_VAL: usize = 9;

fn main() {
    play_game(3);
}


fn play_game(start: usize) {
    let mut ring = Ring::from_str(INPUT);
    let mut current_cup = start;
    for _ in 0..100 {
        // ring.print(current_cup);
        let loop_: HashSet<usize> = (1usize..4).map({
            |idx| ring.nth_next(current_cup, idx)
        }).collect();
        let destination_cup = get_destination(current_cup, loop_);
        ring.move_three(current_cup, destination_cup);
        current_cup = ring.links[current_cup].next.unwrap();
    }

    println!("{}", ring.as_str(ring.nth_next(1, 1)));
}


fn get_destination(current_cup: usize, excluded_cups: HashSet<usize>) -> usize {
    let mut destination_cup = current_cup;
    loop {
        destination_cup = decrement(destination_cup);
        if !excluded_cups.contains(&destination_cup) { break }
    }
    destination_cup
}


fn decrement(val: usize) -> usize {
    if val == MIN_VAL {
        return MAX_VAL;
    }
    val - 1 
}


struct Link {
    previous: Option<usize>,
    next: Option<usize>
}


struct Ring {
    links: Vec<Link>
}


impl Ring {
    fn from_str(input: &str) -> Ring {
        let mut links: Vec<Link> = (0..SIZE + 1).map(|_| Link { previous: None, next: None }).collect();
        let vals: Vec<usize> = input.chars()
            .map(|c| c.to_string().parse::<usize>()
            .unwrap())
            .collect();
        for idx in 1..SIZE {
            links[vals[idx]].previous = Some(vals[idx - 1]);
        }
        for idx in 0..(SIZE - 1) {
            links[vals[idx]].next = Some(vals[idx + 1]);
        }
        links[vals[0]].previous = Some(vals[SIZE - 1]);
        links[vals[SIZE - 1]].next = Some(vals[0]);
        Ring { links: links }
    }

    fn as_vec(&self, start: usize) -> Vec<usize> {
        let mut output: Vec<usize> = vec![];
        output.push(start);
        let mut val = start;
        loop {
            val = self.links[val].next.unwrap();
            if val == start { break; }
            output.push(val);
        }
        output
    }

    fn print(&self, start: usize) {
        println!("{}",
            self.as_vec(start).iter().map(
                |v| v.to_string()
            ).collect::<Vec<String>>().join(" ")
        );
    }

    fn nth_next(&self, start: usize, n: usize) -> usize {
        let mut val = start;
        for _ in 0..n {
            val = self.links[val].next.unwrap();
        }
        val
    }

    fn move_three(&mut self, start: usize, end: usize) {
        // before: start -> loop_left -> ... -> loop_right -> x
        //  after: start -> x
        
        // before: end -> y
        //  after: end -> loop_left -> ... -> loop_right -> y
        let loop_left: usize = self.nth_next(start, 1);
        let loop_right: usize = self.nth_next(start, 3);
        let x: usize = self.links[loop_right].next.unwrap();

        self.links[start].next = Some(x);
        self.links[x].previous = Some(start);

        let y: usize = self.links[end].next.unwrap();

        self.links[y].previous = Some(loop_right);
        self.links[loop_right].next = Some(y);
        self.links[loop_left].previous = Some(end);
        self.links[end].next = Some(loop_left);
    }

    fn as_str(&self, start: usize) -> String {
        self.as_vec(start).iter()
            .map(|v| v.to_string()).collect::<Vec<String>>().join("")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let ring = Ring::from_str(INPUT);
        assert!(ring.links[3].next.unwrap() == 8);
        assert!(ring.links[3].previous.unwrap() == 7);
        assert!(ring.links[7].previous.unwrap() == 6);
        assert!(ring.links[7].next.unwrap() == 3);
        assert!(ring.links[2].next.unwrap() == 5);
        assert!(ring.links[2].previous.unwrap() == 1);
    }

    fn test_as_vec() {
        let ring = Ring::from_str(INPUT);
        assert_eq!(ring.as_vec(3), vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
        assert_eq!(ring.as_vec(2), vec![2, 5, 4, 6, 7, 3, 8, 9, 1]);
    }

    fn test_nth_next() {
        let ring = Ring::from_str(INPUT);
        assert_eq!(ring.nth_next(3, 0), 3);
        assert_eq!(ring.nth_next(3, 1), 8);
        assert_eq!(ring.nth_next(3, 2), 9);
    }

    fn test_move_three() {
        let mut ring = Ring::from_str(INPUT);
        ring.move_three(3, 3);
        assert_eq!(ring.as_vec(3), vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);
    }
}