mod aoc;

use self::aoc::get_string_rows;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Guard {
    id: u32,
    number_of_minutes_asleep: u32,
    minutes_asleep: Vec<u32>
}

impl Guard {
    fn mark_sleeping_minutes(&mut self, start: &LogEntry, end: &LogEntry) {
        let minutes = end.minutes_between(start);
        for i in start.minutes..(start.minutes + minutes) {
            match self.minutes_asleep.get_mut(i as usize) {
                Some(min) => {
                    *min = *min + 2;
                },
                None => { }
            };
        }
    }
}

impl Eq for Guard { }
impl PartialEq for Guard {
    fn eq(&self, other: &Guard) -> bool {
        self.id == other.id
    }
}

impl Ord for Guard {
    fn cmp(&self, other: &Guard) -> Ordering {
        self.number_of_minutes_asleep.cmp(&other.number_of_minutes_asleep)
    }
}

impl PartialOrd for Guard {
    fn partial_cmp(&self, other: &Guard) -> Option<Ordering> {
        Some(self.number_of_minutes_asleep.cmp(&other.number_of_minutes_asleep))
    }
}

#[derive(Debug, Eq)]
enum LogEvent{
    ShiftStart(u32),
    FallsAsleep,
    WakeUp
}

impl PartialEq for LogEvent {
    fn eq(&self, other: &LogEvent) -> bool {
        use self::LogEvent::*;

        match (self, other) {
            (ShiftStart(a), ShiftStart(b)) => a == b,
            _ => self == other
        }
    }
}

#[derive(Debug, Eq)]
struct LogEntry {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minutes: u32,
    event: LogEvent
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        if self.year.cmp(&other.year) == Ordering::Equal {
            if self.month.cmp(&other.month) == Ordering::Equal {
                if self.day.cmp(&other.day) == Ordering::Equal {
                    if self.hour.cmp(&other.hour) == Ordering::Equal {
                        self.minutes.cmp(&other.minutes)
                    }
                    else {
                        self.hour.cmp(&other.hour)
                    }
                }
                else {
                    self.day.cmp(&other.day)
                }
            }
            else {
                self.month.cmp(&other.month)
            }
        }
        else {
            self.year.cmp(&other.year)
        }
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day &&
        self.hour == other.hour &&
        self.minutes == other.minutes &&
        self.event == other.event
    }
}

impl LogEntry {
    fn minutes_between(&self, other: &LogEntry) -> u32 {
        (self.day - other.day) * 24 * 60 +
        (self.hour - other.hour) * 60 +
        (self.minutes - other.minutes)
    }
}

fn parse_log_to_entries(log: Vec<String>) -> Vec<LogEntry> {
    let mut log_entries = Vec::new();

    for line in log {
        let values: Vec<String> = line
            .split(|c| "[- :]#".contains(c))
            .filter(|x| !x.is_empty())
            .map(|s| s.to_string())
            .collect();

        let event = match values.len() {
            7 => {
                //Fall asleep or wake up
                if values[5] == "wakes" {
                    LogEvent::WakeUp
                }
                else {
                    LogEvent::FallsAsleep
                }
            }
            9 => {
                //shift start
                let id = values[6].parse::<u32>().expect("err");
                LogEvent::ShiftStart(id)
            }
            _ => { panic!("ERROR!") }
        };


        log_entries.push(
            LogEntry{
                year: values[0].parse::<u32>().expect("ERR"),
                month: values[1].parse::<u32>().expect("ERR"),
                day: values[2].parse::<u32>().expect("ERR"),
                hour: values[3].parse::<u32>().expect("ERR"),
                minutes: values[4].parse::<u32>().expect("ERR"),
                event
            })

    }
    log_entries.sort();
    log_entries
}

fn parse_entries_to_guards(entries: Vec<LogEntry>) -> HashMap<u32, Guard> {
    let mut guards = HashMap::new();
    let mut fell_asleep_entry: Option<LogEntry> = None;
    let mut current_id = 0;
    for entry in entries {
        match entry.event {
            LogEvent::ShiftStart(id) => {
                if !guards.contains_key(&id) {
                    //A guards shift start
                    let guard = Guard{ id,
                        number_of_minutes_asleep: 0,
                        minutes_asleep: vec![0u32; 60]
                    };
                    guards.insert(guard.id, guard);
                }
                current_id = id;
            },
            LogEvent::FallsAsleep => {
                //Guard falls asleep during shift
                fell_asleep_entry = Some(entry);
            },
            LogEvent::WakeUp => {
                //Guard wakes up again, determine for how long he was asleep
                match &fell_asleep_entry {
                    Some(asleep_entry) => {
                        let time_asleep = entry.minutes_between(&asleep_entry);
                        let ref mut guard = guards.get_mut(&current_id).unwrap();
                        guard.number_of_minutes_asleep += time_asleep;
                        guard.mark_sleeping_minutes(&asleep_entry, &entry);
                    }
                    None => { }
                }
            }
        }
    }
    guards
}

fn main() -> std::io::Result<()> {
    let guard_log_lines = get_string_rows("input_data/day_4");
    let log_entries = parse_log_to_entries(guard_log_lines);
    let guards = parse_entries_to_guards(log_entries);

    let mut guards_vec = Vec::new();
    for guard in guards.values() {
        guards_vec.push(guard);
    }
    guards_vec.sort();
    let sleepiest_guard = guards_vec.last().unwrap();
    let mut highest_minute = 0;
    let mut minute_index = 0;
    for i in 0..60 {
        let min = sleepiest_guard.minutes_asleep.get(i).unwrap();
        if min > &highest_minute {
            minute_index = i;
            highest_minute = *min;
        }
    }
    println!("Day 4 part one answer: {}", (sleepiest_guard.id * minute_index as u32));

    highest_minute = 0;
    minute_index = 0;
    let mut guard_id = 0;

    for guard in guards_vec {
    for i in 0..60 {
        let min = guard.minutes_asleep.get(i).unwrap();
        if min > &highest_minute {
            guard_id = guard.id;
            minute_index = i;
            highest_minute = *min;
        }
    }
    }
    println!("Day 4 part two answer: {}", (guard_id * minute_index as u32));

    Ok(())
}
