//! Utility functions for calculation and data parsing.

/// Calculates hours from a percentage and total weekly hours.
///
/// # Arguments
/// * `percent` - The percentage of time (0-100)
/// * `hours_week` - Total hours in a week
///
/// # Returns
/// The calculated hours based on the percentage
///
/// # Examples
/// ```
/// # use project_app::utils::get_hours;
/// assert_eq!(get_hours(50, 40), 20);
/// assert_eq!(get_hours(100, 40), 40);
/// assert_eq!(get_hours(25, 40), 10);
/// ```
pub fn get_hours(percent: i32, hours_week: i32) -> i32 {
    percent * hours_week / 100
}

/// Parses a cell string in format "name|percentage" into components.
///
/// # Arguments
/// * `person` - A string in format "name|percentage"
///
/// # Returns
/// * `Some((&str, i32))` - Tuple with name and percentage value if valid
/// * `None` - If the format is invalid
///
/// # Examples
/// ```
/// # use project_app::utils::info_cell;
/// assert_eq!(info_cell("John|50"), Some(("John", 50)));
/// assert_eq!(info_cell("Alice|100"), Some(("Alice", 100)));
/// assert_eq!(info_cell("invalid"), None);
/// assert_eq!(info_cell("Bob|invalid"), Some(("Bob", 0)));
/// ```
pub fn info_cell(person: &str) -> Option<(&str, i32)> {
    let mut split = person.split('|');
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hours_basic() {
        assert_eq!(get_hours(50, 40), 20);
        assert_eq!(get_hours(100, 40), 40);
        assert_eq!(get_hours(0, 40), 0);
    }

    #[test]
    fn test_get_hours_edge_cases() {
        assert_eq!(get_hours(25, 40), 10);
        assert_eq!(get_hours(75, 40), 30);
        assert_eq!(get_hours(10, 40), 4);
    }

    #[test]
    fn test_info_cell_valid() {
        assert_eq!(info_cell("John|50"), Some(("John", 50)));
        assert_eq!(info_cell("Alice|100"), Some(("Alice", 100)));
        assert_eq!(info_cell("Bob|0"), Some(("Bob", 0)));
    }

    #[test]
    fn test_info_cell_invalid_format() {
        assert_eq!(info_cell("invalid"), None);
        assert_eq!(info_cell("no|separator|here"), None);
        assert_eq!(info_cell(""), None);
    }

    #[test]
    fn test_info_cell_invalid_number() {
        // Should return 0 for invalid numbers
        assert_eq!(info_cell("Bob|invalid"), Some(("Bob", 0)));
        assert_eq!(info_cell("Carol|abc"), Some(("Carol", 0)));
    }

    #[test]
    fn test_info_cell_empty_name() {
        assert_eq!(info_cell("|50"), Some(("", 50)));
    }
}
