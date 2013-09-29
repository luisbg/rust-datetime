// Copyright 2013 Luis de Bethencourt <luis@debethencourt.com>
// Copyright 2013 The Rust Project Developers
// http://rust-lang.org

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
 * Gregorian Calendar module for the datetime library of the Rust programming
 * language
 *
 * Implements a hybrid calendar that supports both the Julian and Gregorian
 * calendar systems.
 *
 * http://en.wikipedia.org/wiki/Gregorian_calendar
 * http://en.wikipedia.org/wiki/Julian_calendar
 */

static YEAR_BASE: int = 1900;
static DAYSPERLYEAR: uint = 366;
static DAYSPERNYEAR: uint = 365;
static DAYSPERWEEK: uint = 7;

pub fn is_leap_year(year: uint) -> bool {
    !(year % 4 != 0) && ((year % 100 != 0) || !(year % 400 != 0))
}

pub fn year_size(year: uint) -> uint {
    if is_leap_year(year) { DAYSPERLYEAR } else { DAYSPERNYEAR }
}


pub struct GCalendar {
    /**
    */
    sec: uint,
    min: uint,
    hour: uint,
    day_of_month: uint,
    month: uint,
    year: uint,
    day_of_week: uint,
    day_of_year: uint
}

impl GCalendar {
    /**
    * Allocates a GCalendar object
    */
    pub fn new() -> GCalendar {
        GCalendar {
            sec: 0,
            min: 0,
            hour: 0,
            day_of_month: 0,
            month: 0,
            year: 0,
            day_of_week: 0,
            day_of_year: 0,
        }
    }

    pub fn new_from_epoch(since_epoch: uint) -> GCalendar {
        let epoch_year = 1970;
        let mut year = epoch_year;

        let millisecs_day = 86400000;

        let mut dayclock = since_epoch % millisecs_day;
        let mut dayno = since_epoch / millisecs_day;

        let hour = dayclock / 3600000;
        dayclock = dayclock - (hour * 3600000);

        let min = dayclock / 60000;
        dayclock = dayclock - (min * 60000);

        let sec = dayclock / 1000;

        while (dayno >= year_size(year)) {
            dayno -= year_size(year);
            year += 1;
        }
        let year = year;

        GCalendar {
            sec: sec,
            min: min,
            hour: hour,
            day_of_month: 0,
            month: 0,
            year: year,
            day_of_week: 0,
            day_of_year: dayno,
        }
    }

    pub fn get_sec(&self) -> uint {
        self.sec
    }

    pub fn get_min(&self) -> uint {
        self.min
    }

    pub fn get_hour(&self) -> uint {
        self.hour
    }

    pub fn get_year(&self) -> uint {
        self.year
    }

    pub fn get_day_of_year(&self) -> uint {
        self.day_of_year
    }

    pub fn get_date(&self) -> ~str {
        // ToDo
        ~""
    }
}

#[cfg(test)]
mod test {
    use super::GCalendar;

    #[test]
    fn first_test() {
        let gc = GCalendar::new_from_epoch(433166421023);
        assert_eq!(gc.get_sec(), 21);
        assert_eq!(gc.get_min(), 0);
        assert_eq!(gc.get_hour(), 12);
        assert_eq!(gc.get_year(), 1983);
        assert_eq!(gc.get_day_of_year(), 265);
    }
}
