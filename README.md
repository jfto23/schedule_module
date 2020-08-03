# Schedule Module

This is a script that parses ics files and shows the current/next course
for the day. It is intended to be used as a module for Polybar.

## Usage

As a standalone, the script simply prints out the current and next course. The
ics file must be in the same folder as the binary (in
`schedule_module/target/debug`). Example:

```
git clone https://github.com/jfto23/schedule_module
cd schedule_module
cargo run

Output:
1. COMP273-001 LEC (ENGMC 204)
2. COMP251-001 LEC (MCMED 522) [14H05]
```

## Polybar

If you want to add this to your bar, create a module for it in polybar's config
file (usually in `~/.config/polybar/config`):

```
[module/current_course]
type = custom/script

exec = 
tail = true
interval = 60

[module/next_course]
type = custom/script

exec = 
tail = true
interval = 60
```

and then add both modules to the bar:

```
modules-left = current_course next_course

```

## Limitations

The script is extremely bare-bones and can only parse a small subset of ics
files. Every course must have a `SUMMARY`, `DTSTART`, `DTEND` and
`RRULE`. Only weekly events are going to work. In other words, the script is only
intended to work for weekly school-like schedules.
