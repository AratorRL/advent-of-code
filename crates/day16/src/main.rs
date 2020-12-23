use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use std::{fs::File, ops::RangeInclusive};

#[derive(Debug)]
struct ValidRange {
    ranges: Vec<RangeInclusive<u32>>,
}

impl ValidRange {
    fn from_text(s: &str) -> Self {
        let ranges = s
            .split(" or ")
            .map(|part| {
                let numbers: Vec<u32> =
                    part.split("-").map(|s| s.parse::<u32>().unwrap()).collect();
                numbers[0]..=numbers[1]
            })
            .collect();
        Self { ranges }
    }

    fn validate(&self, n: u32) -> bool {
        self.ranges.iter().any(|range| range.contains(&n))
    }
}

#[derive(Debug)]
struct TicketRules {
    field_ranges: HashMap<String, ValidRange>,
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>,
}

fn read_rules() -> Result<(TicketRules, Ticket, Vec<Ticket>)> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let groups: Vec<Vec<String>> = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|line| line.to_owned())
        .collect();

    let rules: Vec<(&str, &str)> = groups[0]
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut field_ranges = HashMap::new();
    for (field, ranges) in rules {
        field_ranges.insert(String::from(field), ValidRange::from_text(ranges));
    }
    let ticket_rules = TicketRules { field_ranges };

    let own_ticket: Ticket = Ticket {
        fields: groups[1][1]
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
    };

    let nearby_tickets: Vec<Ticket> = groups[2][1..]
        .iter()
        .map(|line| Ticket {
            fields: line.split(",").map(|s| s.parse::<u32>().unwrap()).collect(),
        })
        .collect();

    Ok((ticket_rules, own_ticket, nearby_tickets))
}

fn part1(rules: &TicketRules, nearby_tickets: &[Ticket]) -> u32 {
    let mut invalid_numbers_sum = 0;
    for ticket in nearby_tickets {
        for value in &ticket.fields {
            if !rules
                .field_ranges
                .iter()
                .any(|(_, range)| range.validate(*value))
            {
                invalid_numbers_sum += value;
            }
        }
    }
    invalid_numbers_sum
}

fn discard_invalid_tickets(rules: &TicketRules, nearby_tickets: Vec<Ticket>) -> Vec<Ticket> {
    let mut not_invalid_tickets = Vec::new();
    for ticket in nearby_tickets {
        if ticket.fields.iter().all(|value| {
            rules
                .field_ranges
                .iter()
                .any(|(_, range)| range.validate(*value))
        }) {
            not_invalid_tickets.push(ticket);
        }
    }
    not_invalid_tickets
}

fn deduce_fields(rules: &TicketRules, nearby_tickets: &[Ticket]) -> HashMap<String, usize> {
    let mut potential_names: Vec<HashSet<String>> = Vec::new();

    let all_names: HashSet<String> = rules
        .field_ranges
        .keys()
        .map(|name| name.to_owned())
        .collect();

    for _ in &nearby_tickets[0].fields {
        potential_names.push(all_names.clone());
    }

    loop {
        for ticket in nearby_tickets {
            for (i, value) in ticket.fields.iter().enumerate() {
                potential_names[i] = rules
                    .field_ranges
                    .iter()
                    .filter(|(_, ranges)| ranges.validate(*value))
                    .map(|(name, _)| name.to_owned())
                    .collect::<HashSet<String>>()
                    .intersection(&potential_names[i])
                    .map(|name| name.to_owned())
                    .collect();

                if potential_names[i].len() == 1 {
                    let name = potential_names[i].iter().next().unwrap().to_owned();
                    for j in 0..potential_names.len() {
                        if j != i {
                            potential_names[j].remove(&name);
                        }
                    }
                    continue;
                }
            }
        }
        if potential_names.iter().all(|names| names.len() == 1) {
            break;
        }
    }

    let mut mapping = HashMap::new();
    for (i, names) in potential_names.iter().enumerate() {
        assert_eq!(names.len(), 1);
        mapping.insert(names.iter().next().unwrap().to_owned(), i);
    }

    mapping
}

fn part2(ticket: &Ticket, mapping: &HashMap<String, usize>) -> u64 {
    let mut product: u64 = 1;
    for (name, index) in mapping {
        if name.starts_with("departure") {
            product *= ticket.fields[*index] as u64;
        }
    }
    product
}

fn main() -> Result<()> {
    let start = Instant::now();

    let (ticket_rules, own_ticket, nearby_tickets) = read_rules()?;
    let result1 = part1(&ticket_rules, &nearby_tickets);
    println!("result1 = {}", result1);

    let filtered_tickets = discard_invalid_tickets(&ticket_rules, nearby_tickets);
    let mapping = deduce_fields(&ticket_rules, &filtered_tickets);
    let result2 = part2(&own_ticket, &mapping);
    println!("result2 = {}", result2);

    println!("Finished in {} us", start.elapsed().as_micros());
    Ok(())
}
