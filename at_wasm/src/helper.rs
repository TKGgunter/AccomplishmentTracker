use accomplishment_tracker_shared::_AccomplishmentData;
use chrono::Datelike;

use std::collections::HashSet;

use crate::log::*;
use crate::N_LEADERSHIP_PRINCIPLES;

pub fn collect_unique_months_by_year(data: &_AccomplishmentData, year: usize) -> [bool; 12] {
    let mut rt = [false; 12];

    for it in data.events {
        if it.date.year() as usize != year {
            continue;
        }
        let index = it.date.month0() as usize;
        rt[index] = true;
    }
    rt
}

pub fn collect_unique_years(data: &_AccomplishmentData) -> HashSet<usize> {
    let mut rt = HashSet::new();

    for it in data.events {
        let year = it.date.year() as usize;
        console_log!("{} {:?}", year, it.date);
        rt.insert(year);
    }
    rt
}

pub fn collect_leadership_statistic(
    data: &_AccomplishmentData,
    year: usize,
    month: usize,
) -> [usize; 17] {
    let mut rt = [0usize; N_LEADERSHIP_PRINCIPLES];

    for it in data.events.iter() {
        if it.date.year() as usize != year {
            continue;
        } // TODO we can use the fact that the data
        if it.date.month0() as usize != month {
            continue;
        } // TODO we can use the fact that the data
          // is ordered
        if it.leadership_principles[0] == it.leadership_principles[1] {
            rt[it.leadership_principles[0] as usize] += 1;
        } else {
            rt[it.leadership_principles[0] as usize] += 1;
            rt[it.leadership_principles[1] as usize] += 1;
        }
    }

    rt
}
