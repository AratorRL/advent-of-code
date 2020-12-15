use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn get_groups_union() -> Result<Vec<HashSet<char>>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|line_group| {
            line_group
                .iter()
                .map(|line| line.chars())
                .flatten()
                .collect::<HashSet<char>>()
        })
        .collect::<Vec<HashSet<char>>>();

    Ok(result)
}

fn get_intersection(strings: &[String]) -> HashSet<char> {
    let char_sets: Vec<HashSet<char>> = strings
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .collect();

    let mut sets_iter = char_sets.iter();
    let intersection = sets_iter
        .next()
        .map(|first_set| {
            sets_iter.fold(first_set.to_owned(), |isect, next_set| {
                isect
                    .intersection(next_set)
                    .map(|c| c.to_owned())
                    .collect::<HashSet<char>>()
            })
        })
        .unwrap_or(HashSet::new());

    intersection
}

fn get_groups_intersection() -> Result<Vec<HashSet<char>>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|line_group| get_intersection(line_group.into()))
        .collect::<Vec<HashSet<char>>>();

    Ok(result)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let union: Vec<HashSet<char>> = get_groups_union()?;
    let union_sum: usize = union.iter().map(|group| group.len()).sum();
    println!("result: {}", union_sum);

    let isect: Vec<HashSet<char>> = get_groups_intersection()?;
    let isect_sum: usize = isect.iter().map(|group| group.len()).sum();
    println!("result: {}", isect_sum);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_set() {
        let s = "abbbccd";
        let set = s.chars().collect::<HashSet<char>>();
        let unique: HashSet<char> = "abcd".chars().collect();
        assert_eq!(set, unique);
    }

    #[test]
    fn flatten_group() {
        let group = vec!["abc", "bc", "adc"];
        let all_chars: Vec<char> = group.iter().map(|line| line.chars()).flatten().collect();
        assert_eq!(all_chars, vec!['a', 'b', 'c', 'b', 'c', 'a', 'd', 'c']);
    }

    fn into_set(v: Vec<char>) -> HashSet<char> {
        v.into_iter().collect()
    }

    fn into_vec_string(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn intersection() {
        let s1 = into_vec_string(vec!["abc", "ac", "a"]);
        assert_eq!(get_intersection(&s1), into_set(vec!['a']));

        let s2 = into_vec_string(vec!["xy", "zyx", "aybx"]);
        assert_eq!(get_intersection(&s2), into_set(vec!['y', 'x']));
    }
}
