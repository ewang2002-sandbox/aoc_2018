use chrono::{NaiveDateTime, Timelike};
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
    let mut guard_most_occurring_min: HashMap<u32, [usize; 60]> = HashMap::new();
    for guard in guards {
        let corr_events: Vec<&Event> = events
            .iter()
            .filter(|&x| x.guard_num == guard && x.event_type != EventType::BeginShift)
            .collect();

        // Index is minutes (for example, time_table[5] = 5 minutes)
        // Value is number of times encountered (for example, if time_table[5] is 3, then we saw 5
        // minutes 3 times).
        let mut time_table: [usize; 60] = [0; 60];
        let mut time_slept: i64 = 0;
        // Pair every two elemenets
        // [Falls asleep, wakes up] [Falls asleep, wakes up] ...
        for i in (1..corr_events.len()).step_by(2) {
            let start_sleep_time = corr_events[i - 1].time;
            let sleep_session = corr_events[i].time - start_sleep_time;
            time_slept += sleep_session.num_minutes();

            let start_sleep_time_i64 = start_sleep_time.minute() as i64;
            for j in start_sleep_time_i64..(start_sleep_time_i64 + sleep_session.num_minutes()) {
                time_table[(j as usize) % 60] += 1;
            }
        }

        guard_most_occurring_min.insert(guard, time_table);
        guard_slept_time.insert(guard, time_slept);
    }

    // Find the laziest guard
    let laziest_guard = guard_slept_time
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .expect("Something went wrong when trying to find max.");

    // Find index corresponding to minute that is most encountered in sleeping process
    let longest_time = guard_most_occurring_min.get(&laziest_guard).unwrap()
        .iter()
        .enumerate()
        .max_by(|(_, v), (_, w)| v.cmp(w))
        .map(|(idx, _)| idx)
        .expect("Something bad happened.");

    println!("{} {}", laziest_guard, longest_time);

    return (laziest_guard * longest_time as u32) as i32;
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