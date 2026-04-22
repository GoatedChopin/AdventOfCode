use chrono::{NaiveDateTime, TimeDelta, Timelike};
use std::{collections::{HashMap, HashSet}, fs};

fn minute_counts_between(start: &NaiveDateTime, end: &NaiveDateTime) -> HashMap<u32, usize> {
    let mut counts = HashMap::new();
    if start >= end {
        return counts;
    }

    let mut current = start.clone();
    let mut current_minute = current.minute();
    while current_minute != 0 && current < *end {
        let min = counts.get_mut(&current_minute);
        match min {
            Some(min) => *min += 1,
            None => {
                counts.insert(current_minute, 1);
            }
        }
        current += TimeDelta::new(60, 0).expect("Bad TimeDelta");
        current_minute = current.minute();
    }

    let mut last = end.clone();
    let mut last_minute = last.minute();
    while last_minute != 0 && last > current {
        let min = counts.get_mut(&last_minute);
        match min {
            Some(min) => *min += 1,
            None => {
                counts.insert(last_minute, 1);
            }
        }
        last -= TimeDelta::new(60, 0).expect("Bad TimeDelta");
        last_minute = last.minute();
    }

    let gap = last - current;

    if gap.num_hours() == 0 {
        return counts;
    }

    let minutes = gap.num_minutes();
    for i in 0..60 {
        let minute_count = counts.get_mut(&i);
        match minute_count {
            Some(c) => {
                *c += minutes as usize;
            }
            None => {
                counts.insert(i, minutes as usize);
            }
        }
    }

    counts
}

#[derive(Clone, Copy)]
struct EventMember {
    datetime: NaiveDateTime,
    guard_id: isize,
}

#[derive(Clone, Copy)]
enum Event {
    BeginsShift(EventMember),
    FallsAsleep(EventMember),
    WakesUp(EventMember),
}

impl Event {
    fn from_str(s: &str, guard_id: Option<isize>) -> Self {
        let (datetime_str, rest) = s.split_once("]").expect("No NaiveDateTime in string");
        // println!("datetime_str: {}", datetime_str);
        let datetime = NaiveDateTime::parse_from_str(datetime_str, "[%Y-%m-%d %H:%M")
            .expect("Bad datetime string");

        let rest_parts: Vec<&str> = rest.trim().split_whitespace().collect();
        match rest_parts[0] {
            "Guard" => {
                let guard_id = rest_parts[1].replace("#", "").parse().unwrap();
                return Self::BeginsShift(EventMember { datetime, guard_id });
            }
            "falls" => {
                return Self::FallsAsleep(EventMember {
                    datetime,
                    guard_id: guard_id.expect(
                        "guard_id must be provided when event type is WakesUp or FallsAsleep",
                    ),
                });
            }
            "wakes" => {
                return Self::WakesUp(EventMember {
                    datetime,
                    guard_id: guard_id.expect(
                        "guard_id must be provided when event type is WakesUp or FallsAsleep",
                    ),
                });
            }
            _ => {}
        }
        panic!("Invalid event: {}", s);
    }

    fn guard_id(self) -> isize {
        match self {
            Event::BeginsShift(event_member) => event_member.guard_id,
            Event::FallsAsleep(event_member) => event_member.guard_id,
            Event::WakesUp(event_member) => event_member.guard_id,
        }
    }

    fn datetime(self) -> NaiveDateTime {
        match self {
            Event::BeginsShift(event_member) => event_member.datetime,
            Event::FallsAsleep(event_member) => event_member.datetime,
            Event::WakesUp(event_member) => event_member.datetime,
        }
    }
}

fn read_input(input_file: &str) -> Vec<Event> {
    let input = fs::read_to_string(input_file).unwrap();
    let mut binding: Vec<&str> = input.lines().collect();
    binding.sort();
    let mut events = vec![];
    let mut current_guard_id = None;
    for line in binding.into_iter() {
        let event = Event::from_str(line, current_guard_id);
        current_guard_id = match event {
            Event::BeginsShift(event_member) => Some(event_member.guard_id),
            Event::FallsAsleep(event_member) => Some(event_member.guard_id),
            Event::WakesUp(event_member) => Some(event_member.guard_id),
        };
        events.push(event);
    }
    events
}

fn get_best_minute(events: &Vec<Event>) -> (i32, usize) {
    let mut minute_counts: HashMap<u32, usize> = HashMap::new();
    let mut current_datetime = events[0].datetime();
    for event in events.iter() {
        match event {
            Event::BeginsShift(_) => {}
            Event::FallsAsleep(event_member) => {
                current_datetime = event_member.datetime;
            }
            Event::WakesUp(event_member) => {
                // println!(
                //     "Guard slept from {} to {}",
                //     current_datetime, event_member.datetime
                // );
                let sub_minute_counts =
                    minute_counts_between(&current_datetime, &event_member.datetime);
                sub_minute_counts.iter().for_each(|(minute, count)| {
                    let mcount = minute_counts.get_mut(minute);
                    match mcount {
                        Some(c) => {
                            // println!("Adding {} to {}", count, c);
                            *c += count;
                        }
                        None => {
                            minute_counts.insert(*minute, *count);
                        }
                    }
                });
            }
        }
    }
    let (best_minute, minute_count) =
        minute_counts
            .iter()
            .fold((0, 0), |acc, (minute, minute_count)| {
                if *minute_count > acc.1 {
                    return (*minute as i32, *minute_count);
                }
                return acc;
            });
    (best_minute, minute_count)
}

fn part_one(events: Vec<Event>) -> i32 {
    let mut guards: HashMap<isize, TimeDelta> = HashMap::new();
    let mut previous_timestamp = events[0].datetime();
    for event in events.iter() {
        let time_diff = event.datetime() - previous_timestamp;
        previous_timestamp = event.datetime();
        match event {
            Event::BeginsShift(_) => {
                let guard_sleep = guards.get_mut(&event.guard_id());
                match guard_sleep {
                    Some(_) => {}
                    None => {
                        guards.insert(
                            event.guard_id(),
                            TimeDelta::new(0, 0).expect("Bad TimeDelta"),
                        );
                    }
                }
            }
            Event::FallsAsleep(_) => {}
            Event::WakesUp(_) => {
                let guard_sleep = guards.get_mut(&event.guard_id());
                match guard_sleep {
                    Some(sleep) => *sleep += time_diff,
                    None => {}
                }
            }
        }
    }
    let mut max_sleeper_guard_id = 0;
    guards.into_iter().fold(
        TimeDelta::new(0, 0).expect("Bad TimeDelta"),
        |acc, (guard_id, time_slept)| {
            if time_slept > acc {
                max_sleeper_guard_id = guard_id;
                return time_slept;
            }
            return acc;
        },
    );

    println!("Max guard sleeper is {}", max_sleeper_guard_id);

    // Compute a per-minute breakdown for this particular guard.
    let mut minute_counts: HashMap<u32, usize> = HashMap::new();
    let filtered_events: Vec<Event> = events
        .into_iter()
        .filter(|event| event.guard_id() == max_sleeper_guard_id)
        .collect();
    let mut current_datetime = filtered_events[0].datetime();
    for event in filtered_events.iter() {
        match event {
            Event::BeginsShift(_) => {}
            Event::FallsAsleep(event_member) => {
                current_datetime = event_member.datetime;
            }
            Event::WakesUp(event_member) => {
                // println!(
                //     "Guard slept from {} to {}",
                //     current_datetime, event_member.datetime
                // );
                let sub_minute_counts =
                    minute_counts_between(&current_datetime, &event_member.datetime);
                sub_minute_counts.iter().for_each(|(minute, count)| {
                    let mcount = minute_counts.get_mut(minute);
                    match mcount {
                        Some(c) => {
                            // println!("Adding {} to {}", count, c);
                            *c += count;
                        }
                        None => {
                            minute_counts.insert(*minute, *count);
                        }
                    }
                });
            }
        }
    }
    let (best_minute, _minute_count) =
        minute_counts
            .iter()
            .fold((0, 0), |acc, (minute, minute_count)| {
                if *minute_count > acc.1 {
                    return (*minute as i32, *minute_count);
                }
                return acc;
            });

    (max_sleeper_guard_id as i32) * best_minute
}

fn part_two(events: Vec<Event>) -> isize {
    let guard_ids: HashSet<isize> = events.iter().map(|event| event.guard_id()).collect();
    let (mut best_guard_id, mut best_minute, mut best_minute_count) = (0, 0, 0);
    for guard_id in guard_ids {
        println!("Checking guard {}", guard_id);
        let filtered_events = events.iter().filter(|event| event.guard_id() == guard_id).map(|event| event.clone()).collect();
        let (minute, count) = get_best_minute(&filtered_events);
        if count > best_minute_count {
            println!("\tNew best: {} slept {} times on minute {}", guard_id, count, minute);
            best_guard_id = guard_id;
            best_minute = minute;
            best_minute_count = count;
        }
    }

    best_guard_id * best_minute as isize
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), 240);
    }

    #[test]
    fn test_part_two() {
        let input = read_input("test.txt");
        assert_eq!(part_two(input), 4455);
    }
}
