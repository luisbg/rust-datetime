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

    pub fn ydhms_diff(&self, year1: uint, yday1: uint, hour1: uint, min1: uint,
                      sec1: uint, year0: uint, yday0: uint, hour0: uint,
                      min0: uint, sec0: uint) -> uint {
        /* Return an integer value measuring (YEAR1-YDAY1 HOUR1:MIN1:SEC1) -
        * (YEAR0-YDAY0 HOUR0:MIN0:SEC0) in seconds.
        */
        // FIXME: Optimize way to calculate intervening leap days
        let mut intervening_leap_days: uint = 0;
        let mut y: uint = year1;
        while (y > year0) {
            if is_leap_year(y) {intervening_leap_days += 1;}
            y -= 1;
        }

        let years = (year1 - year0);
        let days = 365 * years + yday1 - yday0 + intervening_leap_days;
        let hours = 24 * days + hour1 - hour0;
        let minutes = 60 * hours + min1 - min0;
        60 * minutes + sec1 - sec0
    }

    pub fn mktime(&self) -> uint {
        /* Convert a broken down time structure to a simple representation:
        * seconds since Epoch.
        */
        self.ydhms_diff(self.year, self.yday, self.hour, self.min, self.sec,
                        1970, 0, 0, 0, 0)
    }

    pub fn iso_week_days (&self, yday: uint, wday: uint) -> int {
        /* The number of days from the first day of the first ISO week of this
        * year to the year day YDAY with week day WDAY.
        * ISO weeks start on Monday. The first ISO week has the year's first
        * Thursday.
        * YDAY may be as small as yday_minimum.
        */
        let yday: int = yday as int;
        let wday: int = wday as int;
        let iso_week_start_wday: int = 1; /* Monday */
        let iso_week1_wday: int = 4;      /* Thursday */
        let yday_minimum: int = 366;
        /* Add enough to the first operand of % to make it nonnegative. */
        let big_enough_multiple_of_7: int = (yday_minimum / 7 + 2) * 7;

        yday - (yday - wday + iso_week1_wday + big_enough_multiple_of_7) % 7
            + iso_week1_wday - iso_week_start_wday
     }

    pub fn iso_week (&self, ch: char) -> ~str {
        let mut year: uint = self.year;
        let mut days: int = self.iso_week_days (self.yday, self.wday);

        if (days < 0) {
            /* This ISO week belongs to the previous year. */
            year -= 1;
            days = self.iso_week_days (self.yday + (year_size(year)),
                                       self.wday);
        } else {
            let d: int = self.iso_week_days (self.yday - (year_size(year)),
                                             self.wday);
            if (0 <= d) {
                /* This ISO week belongs to the next year. */
                year += 1;
                days = d;
            }
        }

        match ch {
            'G' => format!("{}", year),
            'g' => format!("{:02u}", (year % 100 + 100) % 100),
            'V' => format!("{:02d}", days / 7 + 1),
            _ => ~""
        }
    }

    pub fn get_date(&self, ch: char) -> ~str {
        let die = || format!("strftime: can't understand this format {} ", ch);
        match ch {
            'A' => match self.wday {
                0 => ~"Sunday",
                1 => ~"Monday",
                2 => ~"Tuesday",
                3 => ~"Wednesday",
                4 => ~"Thursday",
                5 => ~"Friday",
                6 => ~"Saturday",
                _ => die()
            },
            'a' => match self.wday {
                0 => ~"Sun",
                1 => ~"Mon",
                2 => ~"Tue",
                3 => ~"Wed",
                4 => ~"Thu",
                5 => ~"Fri",
                6 => ~"Sat",
                _ => die()
            },
            'B' => match self.month {
                1 => ~"January",
                2 => ~"February",
                3 => ~"March",
                4 => ~"April",
                5 => ~"May",
                6 => ~"June",
                7 => ~"July",
                8 => ~"August",
                9 => ~"September",
                10 => ~"October",
                11 => ~"November",
                12 => ~"December",
                _ => die()
            },
            'b' | 'h' => match self.month {
                1 => ~"Jan",
                2 => ~"Feb",
                3 => ~"Mar",
                4 => ~"Apr",
                5 => ~"May",
                6 => ~"Jun",
                7 => ~"Jul",
                8 => ~"Aug",
                9 => ~"Sep",
                10 => ~"Oct",
                11 => ~"Nov",
                12 => ~"Dec",
                _  => die()
            },
            'C' => format!("{:02u}", self.year / 100),
            'c' => {
                format!("{} {} {} {} {}",
                     self.get_date('a'),
                     self.get_date('b'),
                     self.get_date('e'),
                     self.get_date('T'),
                     self.get_date('Y'))
            }
            'D' | 'x' => {
                format!("{}/{}/{}",
                     self.get_date('m'),
                     self.get_date('d'),
                     self.get_date('y'))
            }
            'd' => format!("{:02u}", self.mday),
            'e' => format!("{:2u}", self.mday),
            'f' => format!("{:09u}", self.sec),
            'F' => {
                format!("{}-{}-{}",
                     self.get_date('Y'),
                     self.get_date('m'),
                     self.get_date('d'))
            }
            'G' => self.iso_week ('G'),
            'g' => self.iso_week ('g'),
            'H' => format!("{:02u}", self.hour),
            'I' => {
                let mut h = self.hour;
                if h > 12 { h -= 12 }
                format!("{:02u}", h)
            }
            'j' => format!("{:03u}", self.yday + 1),
            'k' => format!("{:2u}", self.hour),
            'l' => {
                let mut h = self.hour;
                if h == 0 { h = 12 }
                if h > 12 { h -= 12 }
                format!("{:2u}", h)
            }
            'M' => format!("{:02u}", self.min),
            'm' => format!("{:02u}", self.month),
            'n' => ~"\n",
            'P' => if self.hour < 12 { ~"am" } else { ~"pm" },
            'p' => if self.hour < 12 { ~"AM" } else { ~"PM" },
            'R' => {
                format!("{}:{}",
                     self.get_date('H'),
                     self.get_date('M'))
            }
            'r' => {
                format!("{}:{}:{} {}",
                     self.get_date('I'),
                     self.get_date('M'),
                     self.get_date('S'),
                     self.get_date('p'))
            }
            'S' => format!("{:02u}", self.sec),
            's' => format!("{}", self.mktime()),
            'T' | 'X' => {
                format!("{}:{}:{}",
                     self.get_date('H'),
                     self.get_date('M'),
                     self.get_date('S'))
            }
            't' => ~"\t",
            'U' => format!("{:02u}", (self.yday - self.wday + 7) / 7),
            'u' => {
                let i = self.wday;
                (if i == 0 { 7 } else { i }).to_str()
            }
            'V' => self.iso_week ('V'),
            'v' => {
                format!("{}-{}-{}",
                     self.get_date('e'),
                     self.get_date('b'),
                     self.get_date('Y'))
            }
            'W' => format!("{:02u}", (self.yday - (self.wday - 1 + 7) % 7 + 7) / 7),
            'w' => self.wday.to_str(),
            'Y' => self.year.to_str(),
            'y' => format!("{:02u}", self.year % 100),
            'Z' => ~"UTC",
            'z' => ~"-0000",
            '%' => ~"%",
            _   => die()
        }
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
    }

    #[test]
    fn new_from_epoch() {
        let gc = GCalendar::new_from_epoch(433166421023);
        assert_eq!(gc.get_day_of_week(), 5);
        assert_eq!(gc.get_day_of_year(), 265);
    }
}
