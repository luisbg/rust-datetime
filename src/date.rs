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
    * Returns the number of milliseconds since the 1st of January, 1970,
    * 00:00:00 GMT represented by this Date object.
    */
    pub fn getTime(&self) -> uint {
        self.since_epoch
    }
}

#[cfg(test)]
mod test {
    use super::Date;

    #[test]
    fn from_epoch() {
        let d = Date::from_epoch(433166421023);
        assert_eq!(d.getTime(), 433166421023);
    }
}
