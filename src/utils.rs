pub fn get_hours(percent: i32, hours_week: i32) -> i32 {
    percent * hours_week / 100
}

pub fn info_cell(person: &str) -> Option<(&str, i32)> {
    let mut split = person.split("|");
    if split.clone().count() != 2 {
        return None;
    }
    Some((
        split.next().unwrap(),
        split
            .next()
            .map_or(0, |f| f.parse::<i32>().map_or(0, |f| f)),
    ))
}
