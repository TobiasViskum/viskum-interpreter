use std::sync::Mutex;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use std::time::Duration;
use colored::Colorize;

lazy_static! {
    #[derive(Debug)]
    pub static ref FUNCTION_TRACKER: Mutex<IndexMap<String, (Vec<Duration>, u16)>> = Mutex::new(IndexMap::new());
}

#[derive(Clone, Copy)]
enum CodeColors {
    Color1 = 76,
    Color2 = 70,
    Color3 = 220,
    Color4 = 214,
    Color5 = 208,
    Color6 = 202,
    Color7 = 196,
    Color8 = 160,
    Color9 = 124,
    Color10 = 88,
}

impl CodeColors {
    fn new_from_duration(duration: Duration) -> Self {
        let fast_duration_in_ns = Duration::from_micros(500).as_nanos() as f64;

        let nanos = duration.as_nanos() as f64;

        let ratio: f64 = 2.0;

        if nanos < fast_duration_in_ns {
            CodeColors::Color1
        } else if nanos < fast_duration_in_ns * ratio.powf(1.0) {
            CodeColors::Color2
        } else if nanos < fast_duration_in_ns * ratio.powf(2.0) {
            CodeColors::Color3
        } else if nanos < fast_duration_in_ns * ratio.powf(3.0) {
            CodeColors::Color4
        } else if nanos < fast_duration_in_ns * ratio.powf(4.0) {
            CodeColors::Color5
        } else if nanos < fast_duration_in_ns * ratio.powf(5.0) {
            CodeColors::Color6
        } else if nanos < fast_duration_in_ns * ratio.powf(6.0) {
            CodeColors::Color7
        } else if nanos < fast_duration_in_ns * ratio.powf(7.0) {
            CodeColors::Color8
        } else if nanos < fast_duration_in_ns * ratio.powf(8.0) {
            CodeColors::Color9
        } else {
            CodeColors::Color10
        }
    }

    fn to(&self) -> u8 {
        *self as u8
    }
}

pub fn insert_time(name: &str, time: Duration) {
    let name = name.to_string().replace("_", "-");
    let name = name.as_str();
    let mut times = FUNCTION_TRACKER.lock().unwrap();
    if times.contains_key(name) {
        let prev = times.get_mut(name).unwrap();
        prev.0.push(time);
    } else {
        times.insert(name.to_string(), (vec![time], 0));
    }
}

pub fn increment_recursive_calls(name: &str) {
    let name = name.to_string().replace("_", "-");
    let name = name.as_str();
    let mut times = FUNCTION_TRACKER.lock().unwrap();
    if times.contains_key(name) {
        let prev = times.get_mut(name).unwrap();
        prev.1 += 1;
    } else {
        times.insert(name.to_string(), (vec![], 1));
    }
}

pub fn show_profiler() {
    fn style_text(text: String) -> String {
        format!("{}", text.blue())
    }

    fn get_total_time_string(total_time: Duration) -> String {
        let total_time = format!("{:?}", total_time);
        format!("| Total time: {} ", style_text(total_time))
    }
    fn get_average_time_string(average_time: Duration) -> String {
        let average_time = format!("{:?}", average_time);
        format!("| Average time: {} ", style_text(average_time))
    }
    fn get_calls_string(calls: u32) -> String {
        let calls = format!("{}", calls);
        format!("| Calls: {} ", style_text(calls))
    }
    fn get_recursive_calls_string(recursive_calls: u16) -> String {
        let recursive_calls = format!("{}", recursive_calls);
        format!("| Recursive calls: {} ", style_text(recursive_calls))
    }

    println!("\n{}\n", "Profiler report".bold().underline());

    let mut largest_time = Duration::new(0, 0);

    let mut longest_name: usize = 0;
    let mut longest_total_time: usize = 0;
    let mut longest_average_time: usize = 0;
    let mut longest_calls: usize = 0;
    let mut longest_recursive_calls: usize = 0;

    let full_block_length = 100;

    let times = FUNCTION_TRACKER.lock().unwrap();
    for (name, durations) in times.iter() {
        let mut total = Duration::new(0, 0);
        for duration in &durations.0 {
            total += *duration;
        }
        if total > largest_time {
            largest_time = total;
        }
        if name.len() > longest_name {
            longest_name = name.len();
        }

        if get_total_time_string(total).len() > longest_total_time {
            longest_total_time = get_total_time_string(total).len();
        }
        if get_average_time_string(total / (durations.0.len() as u32)).len() > longest_average_time {
            longest_average_time = get_average_time_string(
                total / (durations.0.len() as u32)
            ).len();
        }
        if get_calls_string(durations.1 as u32).len() > longest_calls {
            longest_calls = get_calls_string(durations.1 as u32).len();
        }
        if get_recursive_calls_string(durations.1).len() > longest_recursive_calls {
            longest_recursive_calls = get_recursive_calls_string(durations.1).len();
        }
    }

    for (name, durations) in times.iter() {
        let mut count = 0;
        let mut total = Duration::new(0, 0);
        for duration in &durations.0 {
            total += *duration;
            count += 1;
        }
        let average = total / count;

        println!(
            "{}{} {}{}{}{}{}{}{}{}",
            name.blue(),
            " ".repeat(longest_name - name.len()),
            get_total_time_string(total),
            " ".repeat(longest_total_time - get_total_time_string(total).len()),
            get_average_time_string(average),
            " ".repeat(longest_average_time - get_average_time_string(average).len()),
            get_calls_string(count),
            " ", //.repeat(longest_calls - get_calls_string(count).len()),
            get_recursive_calls_string(durations.1),
            " ".repeat(longest_recursive_calls - get_recursive_calls_string(durations.1).len())
        );
        let block_length = (total.as_nanos() as f64) / (largest_time.as_nanos() as f64);
        println!(
            "{}\n",
            format!(
                "\x1b[48;5;{}m{}\x1b[0m\n",
                CodeColors::new_from_duration(total).to(),
                " ".repeat(((block_length * (full_block_length as f64)) as usize).max(1))
            )
        );
    }
}
