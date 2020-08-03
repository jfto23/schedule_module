use std::fs::File;
use std::io::BufReader;
use std::env;
use std::path::Path;
use chrono::prelude::*;
use std::error::Error;

use ical;

enum DateTimeType {
    NaiveType(NaiveDateTime),
    UtcType(DateTime<Utc>)
}

#[derive(Debug, Copy, Clone)] 
enum Frequency {
    Daily,
    Weekly,
    Monthly
}

#[derive(Default,Debug, Clone)]
struct Course {
    summary: Option<String>,
    dtstart: Option<NaiveDateTime>,
    dtend: Option<NaiveDateTime>,
    frequency: Option<Frequency>,
    until: Option<DateTime<Utc>>,
    days: Option<Vec<Weekday>>
}

fn parse_rrule(rules: Vec<&str>, course: &mut Course) {

    for rule in rules {
        let pairs: Vec<&str> = rule
            .split("=")
            .collect();

        match pairs[0] {
            "FREQ" => {

                match pairs[1] {
                    "WEEKLY" => course.frequency = Some(Frequency::Weekly),
                    _ => {}
                }

            },
            "UNTIL" => {
                if let DateTimeType::UtcType(dt) = parse_ics_datetime(pairs[1]).expect("Couldn't parse date-time") {
                    course.until = Some(dt);
                }
            }

            "INTERVAL" => {},
            "BYDAY" => parse_byday(pairs[1], course),
            _ => {},
        }
    }
}


fn parse_ics_datetime(ics_dt: &str) -> Result<DateTimeType, Box<dyn Error>> {
    let year: i32 = ics_dt[0..4].parse::<i32>()?;
    let month: u32 = ics_dt[4..6].parse::<u32>()?;
    let day: u32 = ics_dt[6..8].parse::<u32>()?;

    let hour: u32 = ics_dt[9..11].parse::<u32>()?;
    let minute: u32 = ics_dt[11..13].parse::<u32>()?;
    let second :u32 = ics_dt[13..15].parse::<u32>()?;

    // utc datetime in ics files have "z" at the end
    if ics_dt.len() == 15 {
        Ok(DateTimeType::NaiveType(NaiveDate::from_ymd(year,month,day).and_hms(hour,minute,second)))
    }
    else {
        Ok(DateTimeType::UtcType(Utc.ymd(year,month,day).and_hms(hour,minute,second)))
    }

}


fn parse_byday(days: &str, course: &mut Course) {

    let days: Vec<&str> = days
        .split(",")
        .collect();

    let mut course_days: Vec<Weekday> = Vec::new();
    for day in days {
        match day {
            "MO" => course_days.push(Weekday::Mon),
            "TU" => course_days.push(Weekday::Tue),
            "WE" => course_days.push(Weekday::Wed),
            "TH" => course_days.push(Weekday::Thu),
            "FR" => course_days.push(Weekday::Fri),
            "SA" => course_days.push(Weekday::Sat),
            "SU" => course_days.push(Weekday::Sun),
            _ => {},
        }
    }
    course.days = Some(course_days);
}

fn fetch_courses() -> Vec<Course> {
    let home = env::var("HOME").expect("Couldn't get HOME");
    let path = format!("{}/.config/polybar/schedule_module/schedule.ics", home);
    let path = Path::new(&path);
    let buf = BufReader::new(File::open(path)
                             .expect("Can't find schedule.ics"));

    let reader = ical::PropertyParser::from_reader(buf);
    let mut courses: Vec<Course> = Vec::new();
    let mut course = Course::default();
    for line in reader {

        if let Ok(property) = line {

            let inner = property.value.unwrap();
            if property.name== "RRULE" {
                let entries: Vec<&str> = inner
                    .split(";")
                    .collect();

                parse_rrule(entries, &mut course);
            }

            else if property.name== "DTSTART" {
                if let DateTimeType::NaiveType(dt) = parse_ics_datetime(&inner).expect("Couldn't parse date-time") {
                    course.dtstart = Some(dt);
                }
            }

            else if property.name== "DTEND" {
                if let DateTimeType::NaiveType(dt) = parse_ics_datetime(&inner).expect("Couldn't parse date-time") {
                    course.dtend = Some(dt);
                }
            }

            else if property.name == "SUMMARY" {
                course.summary = Some(inner);
            }

            if let Course {
                summary: Some(_),
                dtstart:Some(_),
                dtend: Some(_),
                frequency: Some(_),
                until: Some(_),
                days: Some(_),
            } = course {
                courses.push(course.clone());
                course = Course::default();
            }

        }
    }

    courses
}

fn pick_course(courses: &Vec<Course>) {
    let now_dt = Utc::now();

    let now_ndt = Utc::now().with_timezone(&Local).naive_local();
    //let now_ndt = NaiveDate::from_ymd(2020,7,14).and_hms(12,00,00);

    let current_day = now_ndt.weekday();
    //let current_day = Weekday::Tue;
    

    let mut duration = chrono::Duration::max_value();

    let mut next_course = Course::default();

    for course in courses {
        let days = course.days.as_ref();
        let days = days.unwrap();
        for day in days {
            if current_day == *day {
                if now_ndt > course.dtstart.unwrap() && now_dt < course.until.unwrap() {
                    if now_ndt.time() < course.dtend.unwrap().time() && now_ndt.time() > course.dtstart.unwrap().time() {
                        println!("{}", course.summary.as_ref().unwrap());
                    }
                }

                let new_duration = course.dtstart.unwrap().time().signed_duration_since(now_ndt.time());

                if now_dt < course.until.unwrap() && new_duration < duration && new_duration > chrono::Duration::zero() {
                    duration = new_duration;
                    next_course = course.clone();
                }
            }
        }
    }

    if !next_course.summary.is_none() {
        println!("{}[{}]", next_course.summary.unwrap(), next_course.dtstart.unwrap().time());
    }
}

fn main() {
    let courses: Vec<Course> = fetch_courses();
    pick_course(&courses);
}

