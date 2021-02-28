use std::collections::BTreeSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("input.txt");

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let mut ok_set: BTreeSet<_> = BTreeSet::new();
    let mut total_questions = 0;
    for line in INPUT.lines() {
        match line {
            "" => {
                total_questions += ok_set.len();
                ok_set.clear();
            } // end of group
            line => ok_set.extend(line.chars()),
        }
    }
    total_questions += ok_set.len();
    println!("Problem1 {}", total_questions);
}

fn problem2() {
    let mut ok_set: Option<BTreeSet<_>> = None;
    let mut total_questions = 0;
    for line in INPUT.lines() {
        ok_set = match line {
            "" => {
                total_questions += ok_set.map_or(0, |x| x.len());
                None
            }
            line => {
                if let Some(qs) = ok_set {
                    let answers = BTreeSet::from_iter(line.chars());
                    let inters: Vec<_> = qs.intersection(&answers).cloned().collect();
                    Some(BTreeSet::from_iter(inters))
                } else {
                    Some(BTreeSet::from_iter(line.chars()))
                }
            }
        }
    }

    total_questions += ok_set.unwrap().len();
    println!("Problem2 {}", total_questions);
}
