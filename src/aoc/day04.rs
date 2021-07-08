use chrono::{NaiveDateTime, Datelike, Timelike};
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part1(input: &Vec<String>) -> i32 {
    let mut date_event: Vec<(NaiveDateTime, String)> = Vec::new();

    for line in input {
        let date_time = NaiveDateTime::parse_from_str(
            line.split(&['[', ']'][..]).collect::<Vec<_>>()[1],
            "%Y-%m-%d %H:%M"
        ).expect(format!("Error parsing \"{}\"", line).as_str());
        date_event.push((date_time, line.split("] ").collect::<Vec<_>>()[1].parse().unwrap()))
    }

    date_event.sort_by(|a, b| a.cmp(b));

    // Populate events vector
    let mut events: Vec<Event> = Vec::new();
    let mut guards: HashSet<u32> = HashSet::new();
    let mut current_guard = 0;
    for (d, e) in &date_event {
        if e.starts_with("Guard") {
            current_guard = get_guard_id(&e);
            guards.insert(current_guard);
            events.push(Event {
                time: *d,
                guard_num: current_guard,
                event_type: EventType::BeginShift
            });
            continue;
        }

        if e.starts_with("falls") {
            events.push(Event {
                time: *d,
                guard_num: current_guard,
                event_type: EventType::FallAsleep
            });
            continue;
        }

        if e.starts_with("wakes") {
            events.push(Event {
                time: *d,
                guard_num: current_guard,
                event_type: EventType::WakesUp
            });
            continue;
        }
    }

    assert_eq!(date_event.len(), events.len());

    // Now determine how much time was spent sleeping
    let mut guard_slept_time: HashMap<u32, i64> = HashMap::new();
    let mut guard_most_occurring_min: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    for guard in guards {
        let corr_events: Vec<&Event> = events
            .iter()
            .filter(|&x| x.guard_num == guard && x.event_type != EventType::BeginShift)
            .collect();

        let mut time_table: HashMap<u32, u32> = HashMap::new();
        for i in 0..=60 {
            time_table.insert(i, 0);
        }
        guard_most_occurring_min.insert(guard, time_table);

        let mut time_slept: i64 = 0;
        for i in (1..corr_events.len()).step_by(2) {
            time_slept += (corr_events[i].time - corr_events[i - 1].time).num_minutes();

            for j in corr_events[i - 1].time.minute()..=corr_events[i].time.minute() {
                *guard_most_occurring_min.get_mut(&guard)
                    .unwrap()
                    .get_mut(&(j % 60))
                    .unwrap() += 1;
            }
        }
        guard_slept_time.insert(guard, time_slept);
    }

    // Find the laziest guard
    let laziest_guard = guard_slept_time
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .expect("Something went wrong when trying to find max.");

    let longest_time = guard_most_occurring_min.get(&laziest_guard).unwrap()
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .expect("Something bad happened.");

    return (laziest_guard * longest_time) as i32;
}

fn get_guard_id(str: &String) -> u32 {
    return str.split("#")
        .flat_map(|x| x.split(" begins"))
        .collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
}

#[allow(dead_code)]
pub fn part2(input: &Vec<String>) -> i32 {
    return -1;
}

#[derive(Debug)]
struct Event {
    time: NaiveDateTime,
    guard_num: u32,
    event_type: EventType
}

#[derive(Debug)]
#[derive(PartialEq)]
enum EventType {
    BeginShift,
    FallAsleep,
    WakesUp
}