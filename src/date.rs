// Copyright 2013 Luis de Bethencourt <luis@debethencourt.com>
// Copyright 2013 The Rust Project Developers
// http://rust-lang.org

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
 * date module for the datetime library of the Rust programming language
 *
 */

use extra::time;
use gcalendar::GCalendar;

pub struct Date {
    /**
    * Gregorian Calendar
    */
    priv gcal: GCalendar,
    /**
    * Number of milliseconds since the standard base time known as "epoch",
    * namely 1st of January, 1970, 00:00:00 GMT.
    */
    priv since_epoch: uint,
}

impl Date {
    /**
    * Allocates a Date object and initializes it to represent the specified
    * number of milliseconds since epoch.
    */
    pub fn from_epoch(epoch_date: uint) -> Date {
        let cal: GCalendar = GCalendar::new_from_epoch(epoch_date);
        Date {
            gcal: cal,
            since_epoch: epoch_date
        }
    }

    /**
    * Allocates a Date object and initializes it to represent the current time.
    * For now time is in UTC
    */
    pub fn now() -> Date {
        let ts = time::get_time();
        let sec = (ts.sec * 1000) as uint;
        let msec = (ts.nsec / 1000000) as uint;

        Date::from_epoch(sec + msec)
    }

    /**
    * Returns the number of milliseconds since the 1st of January, 1970,
    * 00:00:00 GMT represented by this Date object.
    */
    pub fn get_time(&self) -> uint {
        self.since_epoch
    }

    /**
    * Returns the Calendar object represented by this Date object.
    */
    pub fn get_cal(&self) -> GCalendar {
        self.gcal
    }

    pub fn to_str(&self) -> ~str {
        self.get_cal().get_date()
    }

    pub fn now_str() -> ~str {
        let d = Date::now();
        d.to_str()
    }
}

#[cfg(test)]
mod test {
    use super::Date;

    #[test]
    fn from_epoch() {
        let d = Date::from_epoch(433166421023);
        assert_eq!(d.get_time(), 433166421023);
    }

    #[test]
    fn now() {
        let d = Date::now();
        let iso8601 = Date::now_str();
        println(iso8601);
        assert_eq!(d.get_cal().get_date(), iso8601);
    }
}
