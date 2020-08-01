use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use chrono::Weekday;
use chrono::DateTime;
use chrono::Utc;
use chrono::Local;

use ical;


// currently only suppports Weekly events
#[derive(Debug)]
enum Frequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Default,Debug)]
struct Course {
    summary: Option<String>,
    dtstart: Option<DateTime<Local>>,
    dtend: Option<DateTime<Local>>,
    frequency: Option<Frequency>,
    until: Option<DateTime<Utc>>,
    days: Option<Vec<Weekday>>,
}

fn parse_rrule(rules: Vec<&str>, course: &mut Course) {

    for rule in rules {
        let pairs: Vec<&str> = rule
            .split("=")
            .collect();

        match pairs[0] {
            "FREQ" => println!("freq"),
            "UNTIL" => println!("until"),
            "INTERVAL" => println!("interval"),
            "BYDAY" => println!("byday"),
            _ => {},
        }
    }


}

fn main() {
    let buf = BufReader::new(File::open("/home/jf/Documents/courses/target/debug/CourseScheduleFall2020.ics")
                             .unwrap());

    let reader = ical::PropertyParser::from_reader(buf);

    let courses: Vec<Course> = Vec::new();
    
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
            }
            else if property.name== "DTSTART" {
                //println!("{}", property.value.unwrap());
            }
            else if property.name== "DTEND" {
                //println!("{}", property.value.unwrap());
            }
            else if property.name == "SUMMARY" {
                //println!("{}", property.value.unwrap());
            }
        }
    }
}

