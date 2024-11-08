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


#ifndef BIKRAM_H
#define BIKRAM_H
#include <iostream>

#include <ctime>

#include <cmath>

#include <string>

class bikram {
  private: int Year = 0;
  int Month = -1;
  int Day = 0;

  double YugaRotation_star = 1582237828;
  double YugaRotation_sun = 4320000;
  double YugaCivilDays;
  double PlanetApogee_sun = 77 + (float) 17 / 60;
  double PlanetCircumm_sun = 13 + (float) 50 / 60;
  double rad = 57.2957795; // = 180/pi

  void get_saura_masa_day(long ahar, int * m, int * d);
  int today_saura_masa_first_p(long ahar);
  double get_tslong(long ahar);
  double get_julian_date(int year, int month, int day);
  void fromJulianDate(double julian_date, int & year, int & month, int & day);

  public: void fromGregorian(int y, int m, int d);
  void fromNepali(int bsYear, int bsMonth, int bsDay);
  void toGregorian(int bsYear, int bsMonth, int bsDay, int & gYear, int & gMonth, int & gDay);
  int getDayOfWeek();
  int getYear();
  int getMonth();
  int getDay();
  std::string getWeekdayName(int year, int month, int day);
  int daysInMonth(int month, int year);
   int daysInGregorianMonth(int gYear, int gMonth); // Declaration here
};

void bikram::get_saura_masa_day(long ahar, int * m, int * d) {
  double tslong_tomorrow;
  int month;
  int day;
  if (today_saura_masa_first_p(ahar)) {
    day = 1;
    tslong_tomorrow = get_tslong(ahar + 1);
    month = (long)(tslong_tomorrow / 30) % 12;
    month = (month + 12) % 12;
  } else {
    get_saura_masa_day(ahar - 1, & month, & day);
    day = day + 1;
  }
  * m = month;
  * d = day;
}

int bikram::today_saura_masa_first_p(long ahar) {
  double tslong_today = get_tslong(ahar);
  double tslong_tomorrow = get_tslong(ahar + 1);
  tslong_today = tslong_today - (int)(tslong_today / 30) * 30;
  tslong_tomorrow = tslong_tomorrow - (int)(tslong_tomorrow / 30) * 30;
  if ((25 < tslong_today) && (tslong_tomorrow < 5)) {
    return 1;
  } else {
    return 0;
  }
}

double bikram::get_tslong(long ahar) {
  double mslong;
  double t1 = (YugaRotation_sun * ahar / YugaCivilDays);
  t1 = t1 - (long) t1;
  mslong = 360 * t1;
  double x1 = mslong - PlanetApogee_sun;
  double y1 = PlanetCircumm_sun / 360;
  double y2 = sin(x1 / rad);
  double y3 = y1 * y2;
  double x2 = asin(y3) * rad;
  double x3 = mslong - x2;
  return x3;
}

double bikram::get_julian_date(int year, int month, int day) {
  if (month <= 2) {
    year -= 1;
    month += 12;
  }
  double a = floor(year / 100.0);
  double b = 2 - a + floor(a / 4.0);
  return floor(365.25 * (year + 4716)) + floor(30.6001 * (month + 1)) + day + b - 1524.5;
}

void bikram::fromJulianDate(double julian_date, int & year, int & month, int & day) {
  int a = static_cast < int > (julian_date + 0.5);
  int b = a + 1537;
  int c = static_cast < int > ((b - 122.1) / 365.25);
  int d = static_cast < int > (365.25 * c);
  int e = static_cast < int > ((b - d) / 30.6001);
  double f = b - d - static_cast < int > (30.6001 * e) + (julian_date + 0.5 - a);
  day = static_cast < int > (f);
  if (e < 14) {
    month = e - 1;
  } else {
    month = e - 13;
  }
  if (month > 2) {
    year = c - 4716;
  } else {
    year = c - 4715;
  }
}

void bikram::fromGregorian(int y, int m, int d) {
  YugaCivilDays = YugaRotation_star - YugaRotation_sun;
  double julian = get_julian_date(y, m, d);
  long ahar = julian - 588465.5;
  int saura_masa_num;
  int saura_masa_day;
  get_saura_masa_day(ahar, & saura_masa_num, & saura_masa_day);
  long YearKali = static_cast < long > (ahar * YugaRotation_sun / YugaCivilDays);
  int YearSaka = YearKali - 3179;
  int nepalimonth = (saura_masa_num) % 12;
  Year = YearSaka + 135 + static_cast < int > ((saura_masa_num - nepalimonth) / 12);
  Month = (saura_masa_num + 12) % 12;
  Day = saura_masa_day;
}

void bikram::toGregorian(int bsYear, int bsMonth, int bsDay, int & gYear, int & gMonth, int & gDay) {
  // Constants used in the conversion
  YugaCivilDays = YugaRotation_star - YugaRotation_sun;

  // Calculate the Saka year from the given Bikram Sambat year
  int YearSaka = bsYear - 135; // Correct year calculation by subtracting 135 instead of 134

  // Calculate the Kali year from the Saka year
  long YearKali = YearSaka + 3179;

  // Estimate the ahar (days since start of Kali Yuga)
  long ahar = static_cast < long > ((YearKali * YugaCivilDays) / YugaRotation_sun);

  // Adjust ahar to get the correct month and day
  int saura_masa_num, saura_masa_day;
  get_saura_masa_day(ahar, & saura_masa_num, & saura_masa_day);

  // Correct the ahar for the input Bikram Sambat month and day
  bsMonth = (bsMonth + 11) % 12; // Adjust month to zero-based index and account for possible negative value

  while (saura_masa_num != bsMonth || saura_masa_day != bsDay) {
    if (saura_masa_num < bsMonth || (saura_masa_num == bsMonth && saura_masa_day < bsDay)) {
      ahar++;
    } else {
      ahar--;
    }
    get_saura_masa_day(ahar, & saura_masa_num, & saura_masa_day);
  }

  // Calculate the Julian date
  double julian_date = ahar + 588465.5;

  // Convert the Julian date to a Gregorian date
  fromJulianDate(julian_date, gYear, gMonth, gDay);
}

void bikram::fromNepali(int bsYear, int bsMonth, int bsDay) {
  toGregorian(bsYear, bsMonth, bsDay, Year, Month, Day);
  Month -= 1; // Adjust for 0-based month
}

int bikram::getDayOfWeek() {
  std::tm timeinfo = {
    0,
    0,
    0,
    Day,
    Month - 1,
    Year - 1900,
    0,
    0,
    0,
    0,
    ""
  };
  std::mktime( & timeinfo);
  return timeinfo.tm_wday;
}

int bikram::getYear() {
  return Year;
}
int bikram::getMonth() {
  return Month + 1;
} // Return 1-based month
int bikram::getDay() {
  return Day;
}

std::string bikram::getWeekdayName(int year, int month, int day) {
  std::tm timeinfo = {
    0,
    0,
    0,
    day,
    month - 1,
    year - 1900,
    0,
    0,
    0,
    0,
    ""
  };
  std::mktime( & timeinfo);
  const char * weekday[] = {
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
  };
  return weekday[timeinfo.tm_wday];
}


int bikram::daysInMonth(int bsYear, int bsMonth) {
  int gYear, gMonth, gDay;
  int nextMonth = (bsMonth % 12) + 1;
  int nextYear = (bsMonth == 12) ? bsYear + 1 : bsYear;

  toGregorian(bsYear, bsMonth, 1, gYear, gMonth, gDay);
  double julian_date_start = get_julian_date(gYear, gMonth, gDay);

  toGregorian(nextYear, nextMonth, 1, gYear, gMonth, gDay);
  double julian_date_end = get_julian_date(gYear, gMonth, gDay);

  return static_cast<int>(julian_date_end - julian_date_start);
}


int bikram::daysInGregorianMonth(int gYear, int gMonth) {
  // Calculate number of days in the given Gregorian month
  switch (gMonth) {
  case 1: case 3: case 5: case 7: case 8: case 10: case 12:
    return 31;
  case 4: case 6: case 9: case 11:
    return 30;
  case 2:
    // Check for leap year
    if ((gYear % 4 == 0 && gYear % 100 != 0) || (gYear % 400 == 0)) {
      return 29;
    } else {
      return 28;
    }
  default:
    return 0; // Invalid month
  }
}

#endif // BIKRAM_H
