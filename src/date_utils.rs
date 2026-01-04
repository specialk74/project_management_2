use chrono::{Datelike, NaiveDate, Utc};

use crate::models::{DayDto, Devs, EffortByDateDto};

pub fn local_to_days(dt: &NaiveDate) -> i32 {
    dt.to_epoch_days()
}

pub fn days_to_local(days: i32) -> NaiveDate {
    NaiveDate::from_epoch_days(days).unwrap()
}

pub fn primo_giorno_settimana_corrente(data: &chrono::NaiveDate) -> chrono::NaiveDate {
    let giorni_da_lunedi = data.weekday().num_days_from_monday();
    *data - chrono::Duration::days(giorni_da_lunedi as i64)
}

pub fn weeks_list(start_date: &chrono::NaiveDate, end_date: &chrono::NaiveDate) -> Vec<DayDto> {
    let mut weeks: Vec<DayDto> = Vec::new();

    let mut start_week = primo_giorno_settimana_corrente(start_date);
    let end_week = primo_giorno_settimana_corrente(end_date);

    while start_week < end_week {
        weeks.push(DayDto::new(local_to_days(&start_week)));
        start_week += chrono::Duration::days(7);
        start_week = primo_giorno_settimana_corrente(&start_week);
    }

    weeks
}

pub fn get_default_weeks() -> (i64, i32, i32) {
    let num_weeks = 52;
    let start_date = local_to_days(&primo_giorno_settimana_corrente(&Utc::now().date_naive()));
    let end_date = local_to_days(&primo_giorno_settimana_corrente(
        &(Utc::now().date_naive() + chrono::Duration::weeks(num_weeks)),
    ));
    (num_weeks, start_date, end_date)
}

pub fn get_weeks(dev: &Devs, project: i32, num_weeks: i64) -> Vec<EffortByDateDto> {
    let mut ret = vec![];
    let mut start_week = local_to_days(&primo_giorno_settimana_corrente(&Utc::now().date_naive()));
    for _ in 0..num_weeks {
        ret.push(EffortByDateDto {
            week: start_week,
            effort: 0,
            dev: *dev,
            project,
            remains: 0,
            total: 0,
            persons: vec!["".to_string()],
        });
        start_week += 7;
    }
    ret
}
