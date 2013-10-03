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

use std::io;
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

    /**
    * Formats the represented time according to the format string.
    */
    pub fn strftime(&self, format: &str) -> ~str {
        let mut buf = ~"";

        do io::with_str_reader(format) |rdr| {
            while !rdr.eof() {
                match rdr.read_char() {
                    '%' => buf.push_str(self.get_cal().get_date(rdr.read_char())),
                    ch => buf.push_char(ch)
                }
            }
        }

        buf
    }

    /**
    * Formats the current time according to the format string.
    */
    pub fn now_strftime(format: &str) -> ~str {
        let d = Date::now();
        d.strftime(format)
    }

    /**
     * Returns a time string formatted according to ISO 8601.
     *
     * utc:   "2012-02-22T14:53:18Z"
     */
    pub fn iso_format(&self) -> ~str {
        self.strftime("%Y-%m-%d %H:%M:%S")
    }

    /**
     * Return a string of the current time in the form
     * "Thu Jan  1 00:00:00 1970".
     */
    pub fn ctime(&self) -> ~str {
        self.strftime("%c")
    }

    /**
     * Returns a time string formatted according to RFC 822.
     *
     * utc:   "Thu, 22 Mar 2012 14:53:18 UTC"
     */
    pub fn rfc822(&self) -> ~str {
        self.strftime("%a, %d %b %Y %T UTC")
    }

    /**
     * Returns a time string formatted according to RFC 822 with Zulu time.
     *
     * utc:   "Thu, 22 Mar 2012 14:53:18 -0000"
     */
    pub fn rfc822z(&self) -> ~str {
        self.strftime("%a, %d %b %Y %T %z")
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
        println("now: " + Date::now_strftime("%Y-%m-%d %H:%M:%S"));
        assert_eq!(true, true);
    }

    #[test]
    fn test_strftime() {
        let d = Date::from_epoch(1234567890543);
        assert_eq!(d.strftime(""), ~"");
        assert_eq!(d.strftime("%A"), ~"Friday");
        assert_eq!(d.strftime("%a"), ~"Fri");
        assert_eq!(d.strftime("%B"), ~"February");
        assert_eq!(d.strftime("%b"), ~"Feb");
        assert_eq!(d.strftime("%C"), ~"20");
        assert_eq!(d.strftime("%c"), ~"Fri Feb 13 23:31:30 2009");
        assert_eq!(d.strftime("%D"), ~"02/13/09");
        assert_eq!(d.strftime("%d"), ~"13");
        assert_eq!(d.strftime("%e"), ~"13");
        assert_eq!(d.strftime("%f"), ~"000000030");
        assert_eq!(d.strftime("%F"), ~"2009-02-13");
        assert_eq!(d.strftime("%G"), ~"2009");
        assert_eq!(d.strftime("%g"), ~"09");
        assert_eq!(d.strftime("%H"), ~"23");
        assert_eq!(d.strftime("%I"), ~"11");
        assert_eq!(d.strftime("%j"), ~"044");
        assert_eq!(d.strftime("%k"), ~"23");
        assert_eq!(d.strftime("%l"), ~"11");
        assert_eq!(d.strftime("%M"), ~"31");
        assert_eq!(d.strftime("%m"), ~"02");
        assert_eq!(d.strftime("%n"), ~"\n");
        assert_eq!(d.strftime("%P"), ~"pm");
        assert_eq!(d.strftime("%p"), ~"PM");
        assert_eq!(d.strftime("%R"), ~"23:31");
        assert_eq!(d.strftime("%r"), ~"11:31:30 PM");
        assert_eq!(d.strftime("%S"), ~"30");
        assert_eq!(d.strftime("%T"), ~"23:31:30");
        assert_eq!(d.strftime("%t"), ~"\t");
        assert_eq!(d.strftime("%U"), ~"06");
        assert_eq!(d.strftime("%u"), ~"5");
        assert_eq!(d.strftime("%V"), ~"07");
        assert_eq!(d.strftime("%v"), ~"13-Feb-2009");
        assert_eq!(d.strftime("%W"), ~"06");
        assert_eq!(d.strftime("%w"), ~"5");
        assert_eq!(d.strftime("%Y"), ~"2009");
        assert_eq!(d.strftime("%y"), ~"09");
        assert_eq!(d.strftime("%z"), ~"-0000");
        assert_eq!(d.strftime("%%"), ~"%");

        assert_eq!(d.iso_format(), ~"2009-02-13 23:31:30");
        assert_eq!(d.ctime(), ~"Fri Feb 13 23:31:30 2009");
        assert_eq!(d.rfc822z(), ~"Fri, 13 Feb 2009 23:31:30 -0000");
        assert_eq!(d.rfc822z(), ~"Fri, 13 Feb 2009 23:31:30 -0000");
    }
}
