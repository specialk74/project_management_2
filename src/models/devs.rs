//! Development categories and identifiers.

use serde::{Deserialize, Serialize};

/// Development categories for effort tracking.
///
/// Represents different teams or roles in the project.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Devs {
    Mcsw = 0,
    Sms = 1,
    Mvh = 2,
    Hw = 3,
    Ele = 4,
    TestHw = 5,
    TestFw = 6,
    TestSys = 7,
    Pjm = 8,
}

impl From<Devs> for i32 {
    fn from(value: Devs) -> Self {
        match value {
            Devs::Mcsw => 0,
            Devs::Sms => 1,
            Devs::Mvh => 2,
            Devs::Hw => 3,
            Devs::Ele => 4,
            Devs::TestHw => 5,
            Devs::TestFw => 6,
            Devs::TestSys => 7,
            Devs::Pjm => 8,
        }
    }
}

impl From<i32> for Devs {
    fn from(value: i32) -> Self {
        match value {
            0 => Devs::Mcsw,
            1 => Devs::Sms,
            2 => Devs::Mvh,
            3 => Devs::Hw,
            4 => Devs::Ele,
            5 => Devs::TestHw,
            6 => Devs::TestFw,
            7 => Devs::TestSys,
            8 => Devs::Pjm,
            _ => Devs::Mcsw,
        }
    }
}

/// Project identifier wrapper for type safety.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct ProjectId(pub usize);

/// Development category identifier wrapper for type safety.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct DevId(pub usize);
