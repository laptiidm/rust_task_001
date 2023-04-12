use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
//use std :: simd :: SimdFloat;

fn main() {
    let mut input_vec: Vec<OneOrTheOther> = Vec::new();

    for i in 1..=7 {
        let mut input = String::new();
        println!("input your number please...#{}", i);
        io::stdin().read_line(&mut input).expect("mistake -_-");
        match input.trim().parse::<i32>() {
            Ok(num) => input_vec.push(OneOrTheOther::First(num)),
            Err(_) => match input.trim().parse::<f64>() {
                Ok(num) => input_vec.push(OneOrTheOther::Second(num)),
                Err(_) => {
                    println!("__________________!!!!!!!bad try!!!!!!!__________________");
                    println!("__________________you might input i32 or f64__________________");
                    panic!("Something went wrong!");
                }
            },
        }
    }

    let sum = sum_as_f64(&input_vec);
    println!("the sum is {}", sum);

    input_vec.sort();
    for elem in &input_vec {
        println!("{:?}", elem);
    }

    let middle = input_vec.len() / 2;
    if input_vec.len() % 2 == 0 {
        let mid1 = input_vec[middle - 1].to_f64_asso().unwrap();
        let mid2 = input_vec[middle].to_f64_asso().unwrap();
        let median = (mid1 + mid2) as f64 / 2.0;
        println!("the median is {}", median);
    } else {
        let median = input_vec[middle].to_f64_asso().unwrap() as f64;
        println!("median is {}", median);
    }

    let vec_f64_from_one_or_the_other: Vec<f64> = my_vec_to_vec_f64(&input_vec);
    let most_common = most_common_element(&vec_f64_from_one_or_the_other);
    println!("most common value {}", most_common);
}

#[derive(Debug, PartialEq)]
//the construction is not particularly clear, it is necessary to clarify
enum OneOrTheOther {
    First(i32),
    Second(f64),
}
// commentgit
impl Ord for OneOrTheOther {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (OneOrTheOther::First(a), OneOrTheOther::First(b)) => a.cmp(b),
            (OneOrTheOther::Second(a), OneOrTheOther::Second(b)) => a.partial_cmp(b).unwrap(),
            (OneOrTheOther::First(a), OneOrTheOther::Second(b)) => {
                match &(*a as f64).partial_cmp(&b) {
                    Some(Ordering::Less) => Ordering::Less,
                    Some(Ordering::Greater) => Ordering::Greater,
                    Some(Ordering::Equal) => Ordering::Equal,
                    None => Ordering::Equal,
                }
            }
            (OneOrTheOther::Second(a), OneOrTheOther::First(b)) => {
                match &a.partial_cmp(&(*b as f64)) {
                    Some(Ordering::Less) => Ordering::Less,
                    Some(Ordering::Greater) => Ordering::Greater,
                    Some(Ordering::Equal) => Ordering::Equal,
                    None => Ordering::Equal,
                }
            }
        }
    }
}

impl Eq for OneOrTheOther {}

impl PartialOrd for OneOrTheOther {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OneOrTheOther {
    fn to_f64_asso(&self) -> Option<f64> {
        match self {
            OneOrTheOther::First(n) => Some(*n as f64),
            OneOrTheOther::Second(n) => Some(*n),
        }
    }
}

fn sum_as_f64(vecter: &[OneOrTheOther]) -> f64 {
    let sum: f64 = vecter.iter().map(|elem| elem.to_f64_asso().unwrap()).sum();
    sum
}

fn most_common_element(input_vec: &Vec<f64>) -> f64 {
    let mut counts = HashMap::new();

    for &value in input_vec {
        *counts.entry(value.to_bits()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| f64::from_bits(value))
        .unwrap_or(0.0)
}

fn my_vec_to_vec_f64(vecter: &Vec<OneOrTheOther>) -> Vec<f64> {
    vecter
        .iter()
        .map(|elem| elem.to_f64_asso())
        .flatten()
        .collect()
}
