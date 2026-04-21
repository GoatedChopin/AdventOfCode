use chrono::DateTime;
use std::{collections::HashMap, fs};

const MINUTES_TO_SECONDS: isize = 60;
const HOURS_TO_SECONDS: isize = MINUTES_TO_SECONDS * 60;
const DAYS_TO_SECONDS: isize = HOURS_TO_SECONDS * 24;
const MONTHS_TO_SECONDS: isize = DAYS_TO_SECONDS * 30;
const YEARS_TO_SECONDS: isize = MONTHS_TO_SECONDS * 12;

#[derive(Clone, Copy)]
struct TimeStamp {
    year: isize,
    month: isize,
    day: isize,
    hour: isize,
    minute: isize,
}

impl TimeStamp {
    fn from_str(s: &str) -> Self {
        let binding = s
            .replace("[", "")
            .replace("]", "")
            .replace("-", " ")
            .replace(":", " ");
        let parts: Vec<&str> = binding.split_whitespace().collect();

        let year = isize_from_leading_zero(parts[0]);
        let month = isize_from_leading_zero(parts[1]);
        let day = isize_from_leading_zero(parts[2]);
        let hour = isize_from_leading_zero(parts[3]);
        let minute = isize_from_leading_zero(parts[4]);

        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }

    fn diff(self, other: &TimeStamp) -> isize {
        let year_diff = (self.year - other.year) * YEARS_TO_SECONDS;
        let month_diff = (self.month - other.month) * MONTHS_TO_SECONDS;
        let day_diff = (self.day - other.day) * DAYS_TO_SECONDS;
        let hour_diff = (self.hour - other.hour) * HOURS_TO_SECONDS;
        let minute_diff = (self.minute - other.minute) * MINUTES_TO_SECONDS;

        year_diff + month_diff + day_diff + hour_diff + minute_diff
    }

    fn minute_counts_between(self, other: &TimeStamp) -> HashMap<isize, isize> {
        let mut counts = HashMap::new();
        if self >= *other {
            return counts;
        }

        let mut 
        while 

        counts
    }
}

fn isize_from_leading_zero(s: &str) -> isize {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() && chars[i] == '0' {
        i += 1;
    }
    let slice = s.split_at_checked(i);
    match slice {
        Some(parts) => {
            if parts.1.len() == 0 {
                return 0;
            }
            return parts.1.parse().unwrap();
        }
        None => {
            return 0;
        }
    }
}

impl Ord for TimeStamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.year != other.year {
            return self.year.cmp(&other.year);
        }
        if self.month != other.month {
            return self.month.cmp(&other.month);
        }
        if self.day != other.day {
            return self.day.cmp(&other.day);
        }
        if self.hour != other.hour {
            return self.hour.cmp(&other.hour);
        }
        if self.minute != other.minute {
            return self.minute.cmp(&other.minute);
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for TimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.hour.partial_cmp(&other.hour) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minute.partial_cmp(&other.minute)
    }
}

impl PartialEq for TimeStamp {
    fn eq(&self, other: &Self) -> bool {
        return self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute;
    }
}

impl Eq for TimeStamp {}

#[derive(Clone, Copy)]
struct EventMember {
    timestamp: TimeStamp,
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
        let (timestamp_str, rest) = s.split_once("]").expect("No timestamp in string");
        let timestamp = TimeStamp::from_str(timestamp_str);

        let rest_parts: Vec<&str> = rest.trim().split_whitespace().collect();
        match rest_parts[0] {
            "Guard" => {
                let guard_id = rest_parts[1].replace("#", "").parse().unwrap();
                return Self::BeginsShift(EventMember {
                    timestamp,
                    guard_id,
                });
            }
            "falls" => {
                return Self::FallsAsleep(EventMember {
                    timestamp,
                    guard_id: guard_id.expect(
                        "guard_id must be provided when event type is WakesUp or FallsAsleep",
                    ),
                });
            }
            "wakes" => {
                return Self::WakesUp(EventMember {
                    timestamp,
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

    fn timestamp(self) -> TimeStamp {
        match self {
            Event::BeginsShift(event_member) => event_member.timestamp,
            Event::FallsAsleep(event_member) => event_member.timestamp,
            Event::WakesUp(event_member) => event_member.timestamp,
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

fn part_one(events: Vec<Event>) -> isize {
    let mut guards = HashMap::new();
    let mut previous_timestamp = events[0].timestamp();
    for event in events.iter() {
        let time_diff = event.timestamp().diff(&previous_timestamp);
        previous_timestamp = event.timestamp();
        match event {
            Event::BeginsShift(_) => {
                let guard_sleep = guards.get_mut(&event.guard_id());
                match guard_sleep {
                    Some(_) => {},
                    None => {
                        guards.insert(event.guard_id(), 0);
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
    guards.into_iter().fold(0, |acc, (guard_id, time_slept)| {
        if time_slept > acc {
            max_sleeper_guard_id = guard_id;
            return time_slept;
        }
        return acc;
    });
    
    // Compute a per-minute breakdown for this particular guard.
    let mut minute_counts = HashMap::new();
    let filtered_events: Vec<Event> = events.iter().filter(|event| event.guard_id() == max_sleeper_guard_id).collect();
    let mut current_timestamp = filtered_events[0].timestamp();
    for event in filtered_events.iter() {
        match event {
            Event::BeginsShift(event_member) => {},
            Event::FallsAsleep(event_member) => {
                current_timestamp = event_member.timestamp;
            },
            Event::WakesUp(event_member) => {
                event_member.timestamp.for_each_minute_between(current_timestamp, |minute| {
                    let mcount = minute_counts.get_mut(minute);
                    match mcount {
                        Some(c) => { *c += 1 }
                        None => { minute_counts.insert(minute, 1); }
                    }
                });
            },
        }
    }
    let best_minute = 0;
    best_minute;
}

fn main() {
    let input = read_input("input.txt");
    println!("Part one: {}", part_one(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_leading_zeros() {
        let ex1 = "00";
        assert_eq!(isize_from_leading_zero(ex1), 0);
        let ex2 = "01";
        assert_eq!(isize_from_leading_zero(ex2), 1);
        let ex3 = "011";
        assert_eq!(isize_from_leading_zero(ex3), 11);
    }

    #[test]
    fn test_part_one() {
        let input = read_input("test.txt");
        assert_eq!(part_one(input), 240);
    }
}
