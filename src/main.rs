use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone};
use chrono_tz::{America, Tz};
use regex::{Regex, RegexBuilder};
use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let timezone = args.get(1).map_or("America/New_York", |s| s.as_str());
    let tz: Tz = timezone.parse().unwrap_or(America::New_York);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(time_str) => match convert_time(&time_str, tz) {
                Ok(converted) => println!("{}", converted),
                Err(e) => eprintln!("Failed to parse '{}': {}", time_str, e),
            },
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}

#[test]
fn test_convert_time() {
    let input_times = vec![
        "Wed Jun 11, 9:00am-10:30am PST",
        "Wed Jun 11, 1:00pm-2:30pm PST",
        "Thu Jun 12, 10:00am-11:30am PST",
        "Fri Jun 13, 9:30am-11:00am PST",
        "Mon Jun 16, 1:00pm-2:30pm PST",
        "Tue Jun 17, 10:00am-11:30am PST",
    ];

    let expected_patterns = vec![
        Regex::new(r"Wed Jun 11, 9:00am-10:30am PST \(12:00pm-1:30pm E[DS]T\)").unwrap(),
        Regex::new(r"Wed Jun 11, 1:00pm-2:30pm PST \(4:00pm-5:30pm E[DS]T\)").unwrap(),
        Regex::new(r"Thu Jun 12, 10:00am-11:30am PST \(1:00pm-2:30pm E[DS]T\)").unwrap(),
        Regex::new(r"Fri Jun 13, 9:30am-11:00am PST \(12:30pm-2:00pm E[DS]T\)").unwrap(),
        Regex::new(r"Mon Jun 16, 1:00pm-2:30pm PST \(4:00pm-5:30pm E[DS]T\)").unwrap(),
        Regex::new(r"Tue Jun 17, 10:00am-11:30am PST \(1:00pm-2:30pm E[DS]T\)").unwrap(),
    ];

    let tz: Tz = "America/New_York".parse().unwrap();

    for (input, expected) in input_times.iter().zip(expected_patterns.iter()) {
        let result = convert_time(input, tz).unwrap();
        assert!(
            expected.is_match(&result),
            "Expected {} to match pattern {}",
            result,
            expected.as_str()
        );
    }
}

fn convert_time(time_str: &str, target_tz: Tz) -> Result<String, String> {
    let re = RegexBuilder::new(
        r"(\w+\s+\w+\s+\d+),\s*(\d+:\d{2}(?:am|pm))-(\d+:\d{2}(?:am|pm))\s+P[SD]T",
    )
    .case_insensitive(true)
    .build()
    .unwrap();

    let captures = re.captures(time_str).ok_or("Invalid format")?;

    let date_str = &captures[1];
    let start_time = &captures[2];
    let end_time = &captures[3];

    let parts: Vec<&str> = date_str.split_whitespace().collect();
    let month = parts[1];
    let day = parts[2];

    let current_year = chrono::Utc::now().year();
    let start_dt = parse_datetime(current_year, month, day, start_time)?;
    let end_dt = parse_datetime(current_year, month, day, end_time)?;

    let start_converted = start_dt.with_timezone(&target_tz);
    let end_converted = end_dt.with_timezone(&target_tz);

    let tz_short = start_converted.format("%Z").to_string();

    let start_formatted = start_converted
        .format("%l:%M%p")
        .to_string()
        .trim()
        .to_lowercase();
    let end_formatted = end_converted
        .format("%l:%M%p")
        .to_string()
        .trim()
        .to_lowercase();

    Ok(format!(
        "{} ({}-{} {})",
        time_str, start_formatted, end_formatted, tz_short
    ))
}

fn parse_datetime(year: i32, month: &str, day: &str, time: &str) -> Result<DateTime<Tz>, String> {
    let month_num = match month.to_lowercase().as_str() {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => return Err("Invalid month".to_string()),
    };

    let day: u32 = day.parse().map_err(|_| "Invalid day")?;
    let time_str = format!("{} {} {} {}", year, month_num, day, time);
    let format_str = "%Y %m %d %I:%M%p";

    let naive_dt = NaiveDateTime::parse_from_str(&time_str, format_str)
        .map_err(|e| format!("Failed to parse datetime: {}", e))?;
    Ok(America::Los_Angeles
        .from_local_datetime(&naive_dt)
        .single()
        .ok_or("Ambiguous or invalid datetime")?)
}
