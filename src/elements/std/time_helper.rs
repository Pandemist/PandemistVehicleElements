use time::PrimitiveDateTime;

pub fn get_german_weekday_string(dt: PrimitiveDateTime) -> String {
    match dt.weekday() {
        time::Weekday::Monday => "Montag".to_string(),
        time::Weekday::Tuesday => "Dienstag".to_string(),
        time::Weekday::Wednesday => "Mittwoch".to_string(),
        time::Weekday::Thursday => "Donnerstag".to_string(),
        time::Weekday::Friday => "Freitag".to_string(),
        time::Weekday::Saturday => "Samstag".to_string(),
        time::Weekday::Sunday => "Sonntag".to_string(),
    }
}

pub fn get_german_month_string(dt: PrimitiveDateTime) -> String {
    match dt.month() {
        time::Month::January => "Januar".to_string(),
        time::Month::February => "Februar".to_string(),
        time::Month::March => "März".to_string(),
        time::Month::April => "April".to_string(),
        time::Month::May => "Mai".to_string(),
        time::Month::June => "Juni".to_string(),
        time::Month::July => "Juli".to_string(),
        time::Month::August => "August".to_string(),
        time::Month::September => "September".to_string(),
        time::Month::October => "Oktober".to_string(),
        time::Month::November => "November".to_string(),
        time::Month::December => "Dezember".to_string(),
    }
}
