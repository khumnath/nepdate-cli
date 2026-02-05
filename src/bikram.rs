/*
 * Copyright (C) 2024 Khumnath CG
 * Email: nath.khum@gmail.com
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */



 #[allow(dead_code)] // Allow unused fields that are initialized and used for derived values
 pub struct Bikram {
     year: i32,
     month: i32,
     day: i32,
     yuga_rotation_star: f64,
     yuga_rotation_sun: f64,
     yuga_civil_days: f64,
     planet_apogee_sun: f64,
     planet_circum_sun: f64,
 }

 const RAD: f64 = 180.0 / std::f64::consts::PI;
 const BS_START_YEAR: i32 = 2000;
 const JULIAN_EPOCH_2000_BS: f64 = 2430828.5; // 14 April 1943

 include!("./precomputed_data.rs"); // Contains NP_MONTHS_DATA: [[i32; 13]; N], NP_DATA_YEAR_COUNT

 impl Bikram {
     pub fn new() -> Self {
         let yuga_rotation_star = 1582237828.0;
         let yuga_rotation_sun = 4320000.0;
         Self {
             year: 0,
             month: -1,
             day: 0,
             yuga_rotation_star,
             yuga_rotation_sun,
             yuga_civil_days: yuga_rotation_star - yuga_rotation_sun,
             planet_apogee_sun: 77.0 + 17.0 / 60.0,
             planet_circum_sun: 13.0 + 50.0 / 60.0,
         }
     }

     pub fn from_gregorian(&mut self, y: i32, m: i32, d: i32) {
         let julian = self.get_julian_date(y, m, d);
         let mut diff = (julian - JULIAN_EPOCH_2000_BS) as i64;
         let mut bs_year = BS_START_YEAR;
         if diff >= 0 {
             // Ensure we don't go out of bounds for NP_MONTHS_DATA
             while (bs_year - BS_START_YEAR) < NP_DATA_YEAR_COUNT as i32 {
                 for month_idx in 0..12 {
                     let days = NP_MONTHS_DATA[(bs_year - BS_START_YEAR) as usize][month_idx];
                     if diff < days as i64 {
                         self.year = bs_year;
                         self.month = (month_idx + 1) as i32;
                         self.day = (diff + 1) as i32;
                         return;
                     } else {
                         diff -= days as i64;
                     }
                 }
                 bs_year += 1; // Move to the next year
             }
         }
         // Fallback to astronomical if outside precomputed range or diff is negative
         self.from_gregorian_astronomical(y, m, d);
     }

     pub fn to_gregorian(&self, bs_year: i32, bs_month: i32, bs_day: i32) -> (i32, i32, i32) {
         if bs_year >= BS_START_YEAR && (bs_year - BS_START_YEAR) < NP_DATA_YEAR_COUNT as i32 {
             let mut days = 0;
             for y_idx in 0..(bs_year - BS_START_YEAR) {
                 days += NP_MONTHS_DATA[y_idx as usize][12]; // Total days in that BS year
             }
             for m_idx in 0..(bs_month - 1) {
                 days += NP_MONTHS_DATA[(bs_year - BS_START_YEAR) as usize][m_idx as usize];
             }
             days += bs_day - 1;
             let target_julian = JULIAN_EPOCH_2000_BS + days as f64;
             return self.from_julian_date(target_julian);
         }
         self.to_gregorian_astronomical(bs_year, bs_month, bs_day)
     }

     fn from_gregorian_astronomical(&mut self, y: i32, m: i32, d: i32) {
         let julian = self.get_julian_date(y, m, d);
         let ahar = (julian - 588465.5) as i64;
         let (sm_num, sm_day) = self.get_saura_masa_day(ahar);
         let year_kali = ((ahar as f64 * self.yuga_rotation_sun) / self.yuga_civil_days).floor() as i64;
         let year_saka = year_kali - 3179;
         let nepali_month = sm_num % 12;
         self.year = (year_saka + 135 + ((sm_num - nepali_month) / 12) as i64) as i32;
         self.month = ((sm_num + 12) % 12) + 1;
         self.day = sm_day;
     }

     fn to_gregorian_astronomical(&self, bs_year: i32, bs_month: i32, bs_day: i32) -> (i32, i32, i32) {
         let year_saka = bs_year - 135;
         let year_kali = year_saka + 3179;
         let mut ahar = ((year_kali as f64 * self.yuga_civil_days) / self.yuga_rotation_sun).floor() as i64;
         // Removed 'mut' as it's not reassigned after initialization
         let bs_month_zero_indexed = (bs_month + 11) % 12; // Convert to 0-indexed for comparison
         loop {
             let (sm, sd) = self.get_saura_masa_day(ahar);
             if sm == bs_month_zero_indexed && sd == bs_day {
                 break;
             }
             ahar += if sm < bs_month_zero_indexed || (sm == bs_month_zero_indexed && sd < bs_day) { 1 } else { -1 };
         }
         let jd = ahar as f64 + 588465.5;
         self.from_julian_date(jd)
     }

     fn get_saura_masa_day(&self, ahar: i64) -> (i32, i32) {
         if self.today_saura_masa_first_p(ahar) {
             let tslong_tomorrow = self.get_tslong(ahar + 1);
             let month = ((tslong_tomorrow / 30.0) as i32 + 12) % 12;
             (month, 1)
         } else {
             let (prev_month, mut day) = self.get_saura_masa_day(ahar - 1);
             day += 1;
             (prev_month, day)
         }
     }

     fn today_saura_masa_first_p(&self, ahar: i64) -> bool {
         let today = self.get_tslong(ahar) % 30.0;
         let tomorrow = self.get_tslong(ahar + 1) % 30.0;
         // Handle negative modulo results for consistency if any intermediate calculation leads to it
         let today_mod = (today + 30.0) % 30.0;
         let tomorrow_mod = (tomorrow + 30.0) % 30.0;
         today_mod > 25.0 && tomorrow_mod < 5.0
     }

     fn get_tslong(&self, ahar: i64) -> f64 {
         let mut t1 = (self.yuga_rotation_sun * ahar as f64) / self.yuga_civil_days;
         t1 -= t1.floor();
         let mslong = 360.0 * t1;
         let x1 = mslong - self.planet_apogee_sun;
         let y1 = self.planet_circum_sun / 360.0;
         let y2 = (x1 / RAD).sin();
         let y3 = y1 * y2;
         let x2 = y3.asin() * RAD;
         mslong - x2
     }

     fn get_julian_date(&self, y: i32, m: i32, d: i32) -> f64 {
         let (mut y, mut m) = (y, m);
         if m <= 2 {
             y -= 1;
             m += 12;
         }
         let a = (y as f64 / 100.0).floor();
         let b = 2.0 - a + (a / 4.0).floor();
         (365.25 * (y as f64 + 4716.0)).floor()
             + (30.6001 * (m as f64 + 1.0)).floor()
             + d as f64 + b - 1524.5
     }

     fn from_julian_date(&self, jd: f64) -> (i32, i32, i32) {
         let z = (jd + 0.5).floor() as i32;
         let a = if z < 2299161 {
             z
         } else {
             let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i32;
             z + 1 + alpha - (alpha / 4)
         };
         let b = a + 1524;
         let c = ((b as f64 - 122.1) / 365.25).floor() as i32;
         let d = (365.25 * c as f64).floor() as i32;
         let e = ((b - d) as f64 / 30.6001).floor() as i32;
         let day = b - d - (30.6001 * e as f64) as i32;
         let month = if e < 14 { e - 1 } else { e - 13 };
         let year = if month > 2 { c - 4716 } else { c - 4715 };
         (year, month, day)
     }

     pub fn get_year(&self) -> i32 { self.year }
     pub fn get_month(&self) -> i32 { self.month }
     pub fn get_day(&self) -> i32 { self.day }

     pub fn days_in_month(&self, bs_year: i32, bs_month: i32) -> i32 {
         if bs_year >= BS_START_YEAR && (bs_year - BS_START_YEAR) < NP_DATA_YEAR_COUNT as i32 {
             NP_MONTHS_DATA[(bs_year - BS_START_YEAR) as usize][(bs_month - 1) as usize]
         } else {
             let (gy1, gm1, gd1) = self.to_gregorian(bs_year, bs_month, 1);
             let (gy2, gm2, gd2) = if bs_month == 12 {
                 self.to_gregorian(bs_year + 1, 1, 1)
             } else {
                 self.to_gregorian(bs_year, bs_month + 1, 1)
             };
             let jd1 = self.get_julian_date(gy1, gm1, gd1);
             let jd2 = self.get_julian_date(gy2, gm2, gd2);
             (jd2 - jd1) as i32
         }
     }

 }
