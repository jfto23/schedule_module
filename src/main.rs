use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use chrono::{Weekday, TimeZone, DateTime, Utc, NaiveDate, NaiveDateTime};

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

                //TODO implement other frequencies
                match pairs[1] {
                    "WEEKLY" => course.frequency = Some(Frequency::Weekly),
                    _ => {}
                }

            },
            "UNTIL" => {
                if let DateTimeType::UtcType(dt) = parse_ics_datetime(pairs[1]) {
                    course.until = Some(dt);
                }
            }

            "INTERVAL" => {},
            "BYDAY" => parse_byday(pairs[1], course),
            _ => {},
        }
    }
}


fn parse_ics_datetime(ics_dt: &str) -> DateTimeType {
    let year: i32 = ics_dt[0..4].parse::<i32>().unwrap();
    let month: u32 = ics_dt[4..6].parse::<u32>().unwrap();
    let day: u32 = ics_dt[6..8].parse::<u32>().unwrap();

    let hour: u32 = ics_dt[9..11].parse::<u32>().unwrap();
    let minute: u32 = ics_dt[11..13].parse::<u32>().unwrap();
    let second :u32 = ics_dt[13..15].parse::<u32>().unwrap();

    // utc datetime in ics files have "z" at the end
    if ics_dt.len() == 15 {
        DateTimeType::NaiveType(NaiveDate::from_ymd(year,month,day).and_hms(hour,minute,second))
    }
    else {
        DateTimeType::UtcType(Utc.ymd(year,month,day).and_hms(hour,minute,second))
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
    let buf = BufReader::new(File::open("/home/jf/Documents/courses/target/debug/CourseScheduleFall2020.ics")
                             .unwrap());

    let reader = ical::PropertyParser::from_reader(buf);

    let mut courses: Vec<Course> = Vec::new();

    let mut course = Course::default();


    for line in reader {

        if let Ok(property) = line {
            //println!("{}", property);

            if property.name== "RRULE" {
                //println!("{}", property.value.unwrap());

                let inner = property.value.unwrap();
                let entries: Vec<&str> = inner
                    .split(";")
                    .collect();

                parse_rrule(entries, &mut course);

                //println!("{:?}", course);
            }
            else if property.name== "DTSTART" {
                //println!("{}", property.value.unwrap());

                if let DateTimeType::NaiveType(dt) = parse_ics_datetime(&property.value.unwrap()) {
                    course.dtstart = Some(dt);
                }
            }
            else if property.name== "DTEND" {
                //println!("{}", property.value.unwrap());
                if let DateTimeType::NaiveType(dt) = parse_ics_datetime(&property.value.unwrap()) {
                    course.dtend = Some(dt);
                }
            }
            else if property.name == "SUMMARY" {
                //println!("{}", property.value.unwrap());
                course.summary = Some(property.value.unwrap());
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

fn main() {
    let courses: Vec<Course> = fetch_courses();
    println!("{:?}", courses);
}

