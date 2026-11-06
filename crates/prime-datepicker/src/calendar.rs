//! Чистая календарная логика DatePicker (по `datepicker.ts` PrimeNG, без DOM и без внешних библиотек дат).
//!
//! Месяц хранится числом 0–11, как в JavaScript `Date`; день недели — 0 (воскресенье) – 6.

use std::cmp::Ordering;

/// Календарная дата без времени и часового пояса.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CalDate {
    /// Год.
    pub year: i32,
    /// Месяц 0–11.
    pub month: u32,
    /// День 1–31.
    pub day: u32,
}

impl CalDate {
    /// Дата; несуществующий день не проверяется (см. [`CalDate::is_valid`]).
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Дата существует в календаре.
    pub fn is_valid(&self) -> bool {
        self.month < 12 && self.day >= 1 && self.day <= days_in_month(self.month, self.year)
    }

    /// Номер дня от 1970-01-01.
    pub fn days(&self) -> i64 {
        days_from_civil(self.year, self.month + 1, self.day)
    }

    /// Дата по номеру дня от 1970-01-01.
    pub fn from_days(days: i64) -> Self {
        let (y, m, d) = civil_from_days(days);
        Self { year: y, month: m - 1, day: d }
    }

    /// День недели: 0 — воскресенье.
    pub fn weekday(&self) -> u32 {
        (self.days() + 4).rem_euclid(7) as u32
    }
}

impl PartialOrd for CalDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CalDate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

/// Високосный год.
pub fn is_leap(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Число дней в месяце (`getDaysCountInMonth`).
pub fn days_in_month(month: u32, year: i32) -> u32 {
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        3 | 5 | 8 | 10 => 30,
        _ => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
    }
}

/// Дни от 1970-01-01 (алгоритм Говарда Хиннанта), месяц 1–12.
fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
    let y = i64::from(if m <= 2 { y - 1 } else { y });
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let mp = i64::from((m + 9) % 12);
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    ((y + i64::from(m <= 2)) as i32, m, d)
}

/// Режим выбора.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectionMode {
    /// Одна дата (по умолчанию).
    #[default]
    Single,
    /// Несколько дат.
    Multiple,
    /// Диапазон: `[начало]` или `[начало, конец]`.
    Range,
}

/// Ячейка дня календаря.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DayCell {
    /// День месяца.
    pub day: u32,
    /// Месяц 0–11.
    pub month: u32,
    /// Год.
    pub year: i32,
    /// День соседнего месяца.
    pub other_month: bool,
    /// Сегодня.
    pub today: bool,
    /// Можно выбрать.
    pub selectable: bool,
}

impl DayCell {
    /// Дата ячейки.
    pub fn date(&self) -> CalDate {
        CalDate::new(self.year, self.month, self.day)
    }
}

/// Ограничения выбора.
#[derive(Clone, Debug, Default)]
pub struct Limits {
    /// Минимальная дата.
    pub min: Option<CalDate>,
    /// Максимальная дата.
    pub max: Option<CalDate>,
    /// Недоступные даты.
    pub disabled_dates: Vec<CalDate>,
    /// Недоступные дни недели (0 — воскресенье).
    pub disabled_days: Vec<u32>,
    /// Разрешён выбор дней соседних месяцев.
    pub select_other_months: bool,
}

impl Limits {
    /// Дату можно выбрать (`isSelectable`).
    pub fn selectable(&self, date: CalDate, other_month: bool) -> bool {
        if other_month && !self.select_other_months {
            return false;
        }
        if self.min.is_some_and(|m| date < m) || self.max.is_some_and(|m| date > m) {
            return false;
        }
        if self.disabled_dates.contains(&date) {
            return false;
        }
        !self.disabled_days.contains(&date.weekday())
    }
}

/// Месяц календаря: сетка недель и номера недель.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Month {
    /// Месяц 0–11.
    pub month: u32,
    /// Год.
    pub year: i32,
    /// Недели по семь ячеек.
    pub weeks: Vec<Vec<DayCell>>,
    /// Номера недель.
    pub week_numbers: Vec<u32>,
}

fn prev_month(month: u32, year: i32) -> (u32, i32) {
    if month == 0 { (11, year - 1) } else { (month - 1, year) }
}

fn next_month(month: u32, year: i32) -> (u32, i32) {
    if month == 11 { (0, year + 1) } else { (month + 1, year) }
}

/// Индекс столбца первого дня месяца с учётом первого дня недели (`getFirstDayOfMonthIndex`).
pub fn first_day_index(month: u32, year: i32, first_day_of_week: u32) -> u32 {
    let sunday_index = if first_day_of_week > 0 { 7 - first_day_of_week } else { 0 };
    let index = CalDate::new(year, month, 1).weekday() + sunday_index;
    if index >= 7 { index - 7 } else { index }
}

/// Номер недели (`getWeekNumber`): по умолчанию неделя ISO, иначе от первого дня года.
pub fn week_number(date: CalDate, first_day_of_week: u32, start_week_from_first_day_of_year: bool) -> u32 {
    let weekday = i64::from(date.weekday());
    let shifted = if start_week_from_first_day_of_year {
        date.days() + 6 + i64::from(first_day_of_week) - weekday
    } else {
        date.days() + 4 - if weekday == 0 { 7 } else { weekday }
    };
    let pivot = CalDate::from_days(shifted);
    let jan1 = CalDate::new(pivot.year, 0, 1);
    ((shifted - jan1.days()) / 7) as u32 + 1
}

/// Сетка месяца (`createMonth`).
pub fn create_month(
    month: u32,
    year: i32,
    today: CalDate,
    first_day_of_week: u32,
    limits: &Limits,
    show_week: bool,
    start_week_from_first_day_of_year: bool,
) -> Month {
    let first = first_day_index(month, year, first_day_of_week);
    let length = days_in_month(month, year);
    let (pm, py) = prev_month(month, year);
    let prev_length = days_in_month(pm, py);
    let (nm, ny) = next_month(month, year);
    let rows = (length + first).div_ceil(7);
    let mut weeks = Vec::new();
    let mut numbers = Vec::new();
    let mut day_no = 1u32;
    let cell = |day: u32, month: u32, year: i32, other: bool| {
        let date = CalDate::new(year, month, day);
        DayCell { day, month, year, other_month: other, today: date == today, selectable: limits.selectable(date, other) }
    };
    for i in 0..rows {
        let mut week = Vec::with_capacity(7);
        if i == 0 {
            for d in (prev_length - first + 1)..=prev_length {
                week.push(cell(d, pm, py, true));
            }
            for _ in 0..(7 - week.len()) {
                week.push(cell(day_no, month, year, false));
                day_no += 1;
            }
        } else {
            for _ in 0..7 {
                if day_no > length {
                    week.push(cell(day_no - length, nm, ny, true));
                } else {
                    week.push(cell(day_no, month, year, false));
                }
                day_no += 1;
            }
        }
        if show_week {
            numbers.push(week_number(week[0].date(), first_day_of_week, start_week_from_first_day_of_year));
        }
        weeks.push(week);
    }
    Month { month, year, weeks, week_numbers: numbers }
}

/// Несколько последовательных месяцев (`createMonths`).
#[allow(clippy::too_many_arguments)]
pub fn create_months(
    month: u32,
    year: i32,
    count: usize,
    today: CalDate,
    first_day_of_week: u32,
    limits: &Limits,
    show_week: bool,
    start_week_from_first_day_of_year: bool,
) -> Vec<Month> {
    (0..count as u32)
        .map(|i| {
            let m = month + i;
            let (m, y) = if m > 11 { (m % 12, year + (month + i) as i32 / 12) } else { (m, year) };
            create_month(m, y, today, first_day_of_week, limits, show_week, start_week_from_first_day_of_year)
        })
        .collect()
}

/// Десять лет «десятилетия», содержащего год (`yearPickerValues`).
pub fn decade_years(year: i32) -> Vec<i32> {
    let base = year - year.rem_euclid(10);
    (0..10).map(|i| base + i).collect()
}

/// В месяце нет ни одной доступной даты (`isMonthDisabled`).
pub fn month_disabled(month: u32, year: i32, limits: &Limits) -> bool {
    !(1..=days_in_month(month, year)).any(|d| limits.selectable(CalDate::new(year, month, d), false))
}

/// В году нет ни одной доступной даты (`isYearDisabled`).
pub fn year_disabled(year: i32, limits: &Limits) -> bool {
    (0..12).all(|m| month_disabled(m, year, limits))
}

/// Выбрана ли дата (`isSelected`).
pub fn is_selected(mode: SelectionMode, value: &[CalDate], date: CalDate) -> bool {
    match mode {
        SelectionMode::Single | SelectionMode::Multiple => value.contains(&date),
        SelectionMode::Range => match value {
            [start, end] => date == *start || date == *end || (*start <= date && date <= *end),
            [start] => date == *start,
            _ => false,
        },
    }
}

/// Классы ячейки дня для режима диапазона: начало и конец, середина.
pub fn range_role(value: &[CalDate], date: CalDate) -> Option<RangeRole> {
    match value {
        [start, end] if date == *start || date == *end => Some(RangeRole::Edge),
        [start, end] if *start <= date && date <= *end => Some(RangeRole::Middle),
        [start] if date == *start => Some(RangeRole::Edge),
        _ => None,
    }
}

/// Роль дня в выбранном диапазоне.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RangeRole {
    /// Начало или конец.
    Edge,
    /// Между началом и концом.
    Middle,
}

/// Новое значение после выбора даты (`onDateSelect` и `selectDate`); `max_count` — предел для `Multiple`.
pub fn select_date(mode: SelectionMode, value: &[CalDate], date: CalDate, max_count: Option<usize>) -> Vec<CalDate> {
    match mode {
        SelectionMode::Single => vec![date],
        SelectionMode::Multiple => {
            if value.contains(&date) {
                value.iter().copied().filter(|d| *d != date).collect()
            } else if max_count.is_none_or(|m| m > value.len()) {
                let mut next = value.to_vec();
                next.push(date);
                next
            } else {
                value.to_vec()
            }
        }
        SelectionMode::Range => match value.first() {
            Some(&start) if value.len() == 1 && date >= start => vec![start, date],
            _ => vec![date],
        },
    }
}

/// Выбор завершён и панель можно закрыть (`hideOnDateTimeSelect`).
pub fn selection_complete(mode: SelectionMode, value: &[CalDate]) -> bool {
    match mode {
        SelectionMode::Single => !value.is_empty(),
        SelectionMode::Multiple => false,
        SelectionMode::Range => value.len() == 2,
    }
}

/// Названия месяцев и дней для форматирования.
#[derive(Clone, Debug)]
pub struct Names {
    /// Полные названия месяцев.
    pub months: Vec<String>,
    /// Краткие названия месяцев.
    pub months_short: Vec<String>,
    /// Полные названия дней недели (с воскресенья).
    pub days: Vec<String>,
    /// Краткие названия дней недели.
    pub days_short: Vec<String>,
    /// Минимальные названия дней недели.
    pub days_min: Vec<String>,
}

impl Default for Names {
    fn default() -> Self {
        let s = |v: &[&str]| v.iter().map(|x| (*x).to_owned()).collect::<Vec<_>>();
        Self {
            months: s(&[
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]),
            months_short: s(&["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]),
            days: s(&["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]),
            days_short: s(&["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]),
            days_min: s(&["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]),
        }
    }
}

/// Форматирование даты (`formatDate`, формат jQuery UI: `d dd o oo D DD m mm M MM y yy`, `'literal'`).
pub fn format_date(date: CalDate, format: &str, names: &Names) -> String {
    let chars: Vec<char> = format.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut literal = false;
    // Сколько подряд идёт символ `c` начиная с позиции `i + 1`: потребляет их.
    let look = |i: &mut usize, c: char| -> bool {
        let hit = *i + 1 < chars.len() && chars[*i + 1] == c;
        if hit {
            *i += 1;
        }
        hit
    };
    let number = |i: &mut usize, c: char, value: i64, len: usize| -> String {
        let mut num = value.to_string();
        if look(i, c) {
            while num.len() < len {
                num.insert(0, '0');
            }
        }
        num
    };
    while i < chars.len() {
        let c = chars[i];
        if literal {
            if c == '\'' && !look(&mut i, '\'') {
                literal = false;
            } else {
                out.push(c);
            }
        } else {
            match c {
                'd' => out.push_str(&number(&mut i, 'd', i64::from(date.day), 2)),
                'D' => {
                    let long = look(&mut i, 'D');
                    let list = if long { &names.days } else { &names.days_short };
                    out.push_str(&list[date.weekday() as usize]);
                }
                'o' => {
                    let day_of_year = date.days() - CalDate::new(date.year, 0, 1).days() + 1;
                    out.push_str(&number(&mut i, 'o', day_of_year, 3));
                }
                'm' => out.push_str(&number(&mut i, 'm', i64::from(date.month) + 1, 2)),
                'M' => {
                    let long = look(&mut i, 'M');
                    let list = if long { &names.months } else { &names.months_short };
                    out.push_str(&list[date.month as usize]);
                }
                'y' => {
                    if look(&mut i, 'y') {
                        out.push_str(&date.year.to_string());
                    } else {
                        let short = date.year.rem_euclid(100);
                        if short < 10 {
                            out.push('0');
                        }
                        out.push_str(&short.to_string());
                    }
                }
                '\'' => {
                    if look(&mut i, '\'') {
                        out.push('\'');
                    } else {
                        literal = true;
                    }
                }
                other => out.push(other),
            }
        }
        i += 1;
    }
    out
}

/// Разбор даты по формату (`parseDate`); `today` задаёт век для двузначного года.
pub fn parse_date(text: &str, format: &str, names: &Names, today: CalDate) -> Option<CalDate> {
    let fmt: Vec<char> = format.chars().collect();
    let src: Vec<char> = text.chars().collect();
    let (mut year, mut month, mut day, mut doy) = (-1i64, -1i64, -1i64, -1i64);
    let mut pos = 0usize;
    let mut literal = false;
    let mut i = 0usize;
    let look = |i: &mut usize, c: char| -> bool {
        let hit = *i + 1 < fmt.len() && fmt[*i + 1] == c;
        if hit {
            *i += 1;
        }
        hit
    };
    let number = |pos: &mut usize, max: usize| -> Option<i64> {
        let start = *pos;
        let mut end = start;
        while end < src.len() && end - start < max && src[end].is_ascii_digit() {
            end += 1;
        }
        if end == start {
            return None;
        }
        *pos = end;
        src[start..end].iter().collect::<String>().parse().ok()
    };
    let name = |pos: &mut usize, long: &[String], short: &[String]| -> Option<usize> {
        // Самое длинное совпадение среди полных и кратких названий, без учёта регистра.
        let mut best: Option<(usize, usize)> = None;
        for (index, candidate) in long.iter().zip(short.iter()).enumerate() {
            for text in [candidate.0, candidate.1] {
                let n = text.chars().count();
                if *pos + n <= src.len()
                    && src[*pos..*pos + n].iter().collect::<String>().eq_ignore_ascii_case(text)
                    && best.is_none_or(|(_, b)| n > b)
                {
                    best = Some((index, n));
                }
            }
        }
        best.map(|(index, n)| {
            *pos += n;
            index
        })
    };
    while i < fmt.len() {
        let c = fmt[i];
        if literal {
            if c == '\'' && !look(&mut i, '\'') {
                literal = false;
            } else {
                if src.get(pos) != Some(&c) {
                    return None;
                }
                pos += 1;
            }
        } else {
            match c {
                'd' => {
                    look(&mut i, 'd');
                    day = number(&mut pos, 2)?;
                }
                'o' => {
                    look(&mut i, 'o');
                    doy = number(&mut pos, 3)?;
                }
                'm' => {
                    look(&mut i, 'm');
                    month = number(&mut pos, 2)?;
                }
                'y' => {
                    let four = look(&mut i, 'y');
                    let raw = number(&mut pos, if four { 4 } else { 2 })?;
                    year = if four {
                        raw
                    } else {
                        let cutoff = i64::from(today.year.rem_euclid(100)) + 10;
                        raw + if raw <= cutoff { 2000 } else { 1900 }
                    };
                }
                'D' => {
                    let long = look(&mut i, 'D');
                    let _ = long;
                    name(&mut pos, &names.days, &names.days_short)?;
                }
                'M' => {
                    look(&mut i, 'M');
                    month = name(&mut pos, &names.months, &names.months_short)? as i64 + 1;
                }
                '\'' => {
                    if look(&mut i, '\'') {
                        if src.get(pos) != Some(&'\'') {
                            return None;
                        }
                        pos += 1;
                    } else {
                        literal = true;
                    }
                }
                other => {
                    if src.get(pos) != Some(&other) {
                        return None;
                    }
                    pos += 1;
                }
            }
        }
        i += 1;
    }
    if pos < src.len() && src[pos..].iter().any(|c| !c.is_whitespace()) {
        return None;
    }
    if year == -1 {
        year = i64::from(today.year);
    }
    if doy > -1 {
        let date = CalDate::new(year as i32, 0, 1);
        let resolved = CalDate::from_days(date.days() + doy - 1);
        return Some(resolved);
    }
    if month < 1 || day < 1 {
        return None;
    }
    let date = CalDate::new(year as i32, (month - 1) as u32, day as u32);
    date.is_valid().then_some(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> CalDate {
        CalDate::new(y, m, day)
    }

    #[test]
    fn weekdays_and_days() {
        // (дата, день недели) — проверено по календарю
        for (date, weekday) in
            [(d(1970, 0, 1), 4), (d(2000, 0, 1), 6), (d(2024, 1, 29), 4), (d(2026, 10, 5), 4), (d(2023, 11, 31), 0), (d(1900, 2, 1), 4)]
        {
            assert_eq!(date.weekday(), weekday, "{date:?}");
        }
        for date in [d(1970, 0, 1), d(1999, 11, 31), d(2024, 1, 29), d(2100, 2, 1), d(1800, 5, 15)] {
            assert_eq!(CalDate::from_days(date.days()), date);
        }
        assert_eq!(d(2000, 0, 1).days() - d(1999, 0, 1).days(), 365);
        assert_eq!(d(2001, 0, 1).days() - d(2000, 0, 1).days(), 366);
    }

    #[test]
    fn month_lengths() {
        for (month, year, want) in
            [(0, 2024, 31), (1, 2024, 29), (1, 2023, 28), (1, 1900, 28), (1, 2000, 29), (3, 2024, 30), (11, 2024, 31)]
        {
            assert_eq!(days_in_month(month, year), want, "{month}/{year}");
        }
        assert!(d(2024, 1, 29).is_valid() && !d(2023, 1, 29).is_valid() && !d(2024, 12, 1).is_valid() && !d(2024, 0, 0).is_valid());
    }

    #[test]
    fn first_day_indexes() {
        // ноябрь 2026 начинается в воскресенье
        assert_eq!(first_day_index(10, 2026, 0), 0);
        assert_eq!(first_day_index(10, 2026, 1), 6, "неделя с понедельника");
        // февраль 2024 начинается в четверг
        assert_eq!(first_day_index(1, 2024, 0), 4);
        assert_eq!(first_day_index(1, 2024, 1), 3);
    }

    #[test]
    fn month_grid() {
        let today = d(2026, 10, 5);
        let m = create_month(10, 2026, today, 0, &Limits::default(), false, false);
        assert_eq!(m.weeks.len(), 5);
        assert!(m.weeks.iter().all(|w| w.len() == 7));
        let first = m.weeks[0][0];
        assert_eq!((first.day, first.month, first.other_month), (1, 10, false), "ноябрь 2026 начинается с воскресенья");
        let last = m.weeks[4][6];
        assert_eq!((last.day, last.month, last.other_month), (5, 11, true), "хвост из декабря");
        assert_eq!(m.weeks.iter().flatten().filter(|c| c.today).count(), 1);
        assert!(m.weeks.iter().flatten().filter(|c| c.other_month).all(|c| !c.selectable), "соседние месяцы недоступны по умолчанию");
        // с понедельника первая неделя начинается с 26 октября
        let m = create_month(10, 2026, today, 1, &Limits::default(), false, false);
        let first = m.weeks[0][0];
        assert_eq!((first.day, first.month, first.other_month), (26, 9, true));
    }

    #[test]
    fn limits_table() {
        let limits = Limits {
            min: Some(d(2026, 10, 10)),
            max: Some(d(2026, 10, 20)),
            disabled_dates: vec![d(2026, 10, 15)],
            disabled_days: vec![0],
            ..Limits::default()
        };
        for (date, want) in [
            (d(2026, 10, 9), false),
            (d(2026, 10, 10), true),
            (d(2026, 10, 15), false),
            (d(2026, 10, 20), true),
            (d(2026, 10, 21), false),
            (d(2026, 10, 16), true),
        ] {
            // 2026-11-15 — воскресенье, 16 — понедельник
            assert_eq!(limits.selectable(date, false), want, "{date:?}");
        }
        assert!(!Limits::default().selectable(d(2026, 10, 1), true));
        assert!(Limits { select_other_months: true, ..Limits::default() }.selectable(d(2026, 10, 1), true));
        assert!(month_disabled(9, 2026, &limits) && !month_disabled(10, 2026, &limits));
        assert!(year_disabled(2025, &limits) && !year_disabled(2026, &limits));
    }

    #[test]
    fn week_numbers() {
        // ISO-недели
        for (date, want) in [(d(2026, 0, 1), 1), (d(2024, 11, 30), 1), (d(2021, 0, 3), 53), (d(2026, 10, 5), 45), (d(2024, 2, 4), 10)] {
            assert_eq!(week_number(date, 0, false), want, "{date:?}");
        }
    }

    #[test]
    fn selection_table() {
        use SelectionMode::*;
        let (a, b, c) = (d(2026, 10, 1), d(2026, 10, 5), d(2026, 10, 9));
        assert_eq!(select_date(Single, &[a], b, None), [b]);
        assert_eq!(select_date(Multiple, &[a], b, None), [a, b]);
        assert_eq!(select_date(Multiple, &[a, b], a, None), [b], "повторный выбор снимает");
        assert!(select_date(Multiple, &[a], a, None).is_empty());
        assert_eq!(select_date(Multiple, &[a, b], c, Some(2)), [a, b], "предел maxDateCount");
        assert_eq!(select_date(Range, &[], a, None), [a]);
        assert_eq!(select_date(Range, &[b], c, None), [b, c]);
        assert_eq!(select_date(Range, &[b], a, None), [a], "конец раньше начала — новое начало");
        assert_eq!(select_date(Range, &[a, b], c, None), [c], "полный диапазон начинается заново");
        assert!(selection_complete(Single, &[a]) && !selection_complete(Single, &[]));
        assert!(!selection_complete(Multiple, &[a, b]));
        assert!(selection_complete(Range, &[a, b]) && !selection_complete(Range, &[a]));
    }

    #[test]
    fn selected_and_range_roles() {
        use SelectionMode::*;
        let (a, b, c) = (d(2026, 10, 1), d(2026, 10, 5), d(2026, 10, 9));
        assert!(is_selected(Single, &[a], a) && !is_selected(Single, &[a], b));
        assert!(is_selected(Multiple, &[a, c], c) && !is_selected(Multiple, &[a, c], b));
        assert!(is_selected(Range, &[a, c], b) && !is_selected(Range, &[a], b));
        assert_eq!(range_role(&[a, c], a), Some(RangeRole::Edge));
        assert_eq!(range_role(&[a, c], c), Some(RangeRole::Edge));
        assert_eq!(range_role(&[a, c], b), Some(RangeRole::Middle));
        assert_eq!(range_role(&[a], a), Some(RangeRole::Edge));
        assert_eq!(range_role(&[a, c], d(2026, 10, 10)), None);
    }

    #[test]
    fn decades() {
        assert_eq!(decade_years(2026), (2020..2030).collect::<Vec<_>>());
        assert_eq!(decade_years(2020)[0], 2020);
        assert_eq!(decade_years(2019)[0], 2010);
    }

    #[test]
    fn format_table() {
        let names = Names::default();
        let date = d(2026, 10, 5);
        for (format, want) in [
            ("mm/dd/yy", "11/05/2026"),
            ("d/m/y", "5/11/26"),
            ("dd.mm.yy", "05.11.2026"),
            ("DD, MM d, yy", "Thursday, November 5, 2026"),
            ("D M yy", "Thu Nov 2026"),
            ("yy-mm-dd", "2026-11-05"),
            ("o", "309"),
            ("oo", "309"),
            ("'day' d", "day 5"),
            ("d 'o''clock'", "5 o'clock"),
        ] {
            assert_eq!(format_date(date, format, &names), want, "{format}");
        }
        assert_eq!(format_date(d(2001, 0, 9), "oo", &names), "009");
        assert_eq!(format_date(d(2005, 0, 9), "y", &names), "05");
    }

    #[test]
    fn parse_table() {
        let names = Names::default();
        let today = d(2026, 10, 5);
        for (text, format, want) in [
            ("11/05/2026", "mm/dd/yy", Some(d(2026, 10, 5))),
            ("1/5/2026", "mm/dd/yy", Some(d(2026, 0, 5))),
            ("05.11.2026", "dd.mm.yy", Some(d(2026, 10, 5))),
            ("Thursday, November 5, 2026", "DD, MM d, yy", Some(d(2026, 10, 5))),
            ("Nov 5 2026", "M d yy", Some(d(2026, 10, 5))),
            ("5/11/30", "d/m/y", Some(d(2030, 10, 5))),
            ("5/11/40", "d/m/y", Some(d(1940, 10, 5))),
            ("309 2026", "o yy", Some(d(2026, 10, 5))),
            ("02/30/2026", "mm/dd/yy", None),
            ("13/01/2026", "mm/dd/yy", None),
            ("hello", "mm/dd/yy", None),
            ("11/05/2026 extra", "mm/dd/yy", None),
            ("11-05-2026", "mm/dd/yy", None),
            ("", "mm/dd/yy", None),
        ] {
            assert_eq!(parse_date(text, format, &names, today), want, "{text} / {format}");
        }
    }

    #[test]
    fn format_parse_round_trip() {
        let names = Names::default();
        let today = d(2026, 10, 5);
        for date in [d(2026, 10, 5), d(2024, 1, 29), d(1999, 11, 31), d(2001, 0, 1)] {
            for format in ["mm/dd/yy", "dd.mm.yy", "yy-mm-dd", "DD, MM d, yy"] {
                let text = format_date(date, format, &names);
                assert_eq!(parse_date(&text, format, &names, today), Some(date), "{text} / {format}");
            }
        }
    }
}
