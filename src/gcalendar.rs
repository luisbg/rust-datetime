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

static YEARBASE: int = 1900;
static DAYSPERLYEAR: uint = 366;
static DAYSPERNYEAR: uint = 365;
static DAYSPERWEEK: uint = 7;
//static DAYSBEFOREMONTH: ~[~[uint]] = [
static DAYSBEFOREMONTH: [[uint, ..13], ..2] = [
    /* Normal years */
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
    /* Leap years */
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366]
];

pub fn is_leap_year(year: uint) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

pub fn year_size(year: uint) -> uint {
    if is_leap_year(year) { DAYSPERLYEAR } else { DAYSPERNYEAR }
}


pub struct GCalendar {
    /*
     * Calendar object with date and time.
     */
    sec: uint,         /* Seconds       [0-59]  */
    min: uint,         /* Minutes       [0-59]  */
    hour: uint,        /* Hours         [0-23]  */
    mday: uint,        /* Day           [0-30]  */
    month: uint,       /* Month         [0-11]  */
    year: uint,        /* Year - 1900           */
    wday: uint,        /* Day of week   [0-6]   */
    yday: uint         /* Days in year  [0-365] */
}

impl GCalendar {
    /**
    * Allocates a GCalendar object at epoch.
    */
    pub fn new_at_epoch() -> GCalendar {
        GCalendar {
            sec: 0,
            min: 0,
            hour: 0,
            mday: 0,
            month: 0,
            year: 0,
            wday: 0,
            yday: 0,
        }
    }

    /**
    * Allocates a GCalendar object at the given date and time.
    */
    pub fn new(sec: uint, min: uint, hour: uint, mday: uint, month: uint,
            year: uint, wday: uint, yday: uint) -> GCalendar {
        GCalendar {
            sec: sec,
            min: min,
            hour: hour,
            mday: mday,
            month: month,
            year: year,
            wday: wday,
            yday: yday,
        }
    }

    /**
    * Allocates a GCalendar object from the milliseconds elapsed since epoch.
    */
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
        let wday = (dayno + 4) % 7;

        while (dayno >= year_size(year)) {
            dayno -= year_size(year);
            year += 1;
        }
        let yday = dayno;

        let ip = DAYSBEFOREMONTH[if is_leap_year(year) {1} else {0}];
        let mut month = 11;
        while (dayno < ip[month]) {
            month -= 1;
        }
        dayno -= ip[month];

        GCalendar {
            sec: sec,
            min: min,
            hour: hour,
            mday: dayno + 1,
            month: month  + 1,
            year: year,
            wday: wday,
            yday: yday,
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

    pub fn get_day_of_month(&self) -> uint {
        self.mday
    }

    pub fn get_month(&self) -> uint {
        self.month
    }

    pub fn get_year(&self) -> uint {
        self.year
    }

    pub fn get_day_of_week(&self) -> uint {
        self.wday
    }

    pub fn get_day_of_year(&self) -> uint {
        self.yday
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
    fn new() {
        let gc = GCalendar::new(21, 0, 12, 23, 9, 1983, 5, 265);
        assert_eq!(gc.get_sec(), 21);
        assert_eq!(gc.get_min(), 0);
        assert_eq!(gc.get_hour(), 12);
        assert_eq!(gc.get_day_of_month(), 23);
        assert_eq!(gc.get_month(), 9);
        assert_eq!(gc.get_year(), 1983);
        assert_eq!(gc.get_day_of_week(), 5);
        assert_eq!(gc.get_day_of_year(), 265);
    }

    #[test]
    fn new_from_epoch() {
        let gc = GCalendar::new_from_epoch(433166421023);
        assert_eq!(gc.get_sec(), 21);
        assert_eq!(gc.get_min(), 0);
        assert_eq!(gc.get_hour(), 12);
        assert_eq!(gc.get_day_of_month(), 23);
        assert_eq!(gc.get_month(), 9);
        assert_eq!(gc.get_year(), 1983);
        assert_eq!(gc.get_day_of_week(), 5);
        assert_eq!(gc.get_day_of_year(), 265);
    }
}
