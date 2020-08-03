# Schedule Module

This is a script that parses ics files and shows the current/next course
for the day. It is intended to be used as a module for Polybar.

## Usage

First, clone the repo and compile the program with `cargo`:

```
git clone https://github.com/jfto23/schedule_module
cd schedule_module
cargo build
```

Then create a new folder in polybar's config folder named `schedule_module` and
put the script in it.

```
cp target/debug/schedule_module ~/.config/polybar/schedule_module

```

Rename your `ics` file to `schedule.ics` and put it in the `schedule_module` folder.


Finally, create two new modules in polybar's config file:

```
[module/current_course]
type = custom/script
exec = $HOME/.config/polybar/schedule_module/./schedule_module | grep \)$
tail = true
interval = 60

[module/next_course]
type = custom/script
exec = $HOME/.config/polybar/schedule_module/./schedule_module | grep ]$
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
