use std::io::stdin;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
struct Person {
    weight: usize,
    height: usize,
}

fn main() {
    let persons = make_persons();
    let mut rankings = Vec::<usize>::new();
    for person in persons.iter() {
        let mut ranking = 1;

        for other in persons.iter() {
            if person == other {
                continue;
            }
            if person.weight < other.weight && person.height < other.height {
                ranking += 1;
            }
        }

        rankings.push(ranking);
    }

    let rankings = rankings.iter()
        .map(|ranking| ranking.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{rankings}");
}

fn make_persons() -> Vec<Person> {
    let num = input_number::<usize>();

    let mut persons = Vec::<Person>::new();

    for index in 0..num {
        let input = input_line_as::<usize>();
        let weight = input[0];
        let height = input[1];

        let person = Person {
            weight,
            height,
        };

        persons.push(person);
    }

    persons
}

fn input_number<T>() -> T
    where
        T: Eq + Ord + Copy + FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
}

fn input_line_as<T>() -> Vec<T>
    where
        T: Eq + Ord + Copy + FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<T>>()
}
