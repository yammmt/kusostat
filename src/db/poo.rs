use chrono::{NaiveDateTime, NaiveTime};
use diesel::{self, prelude::*};
use serde::Serialize;

use crate::schema::poo;
use crate::schema::poo::dsl::poo as all_poos;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Queryable)]
pub struct Poo {
    id: i32,
    pub form: i32,
    pub color: i32,
    pub bleeding: i32,
    pub required_time: NaiveTime,
    pub published_at: NaiveDateTime,
}

impl Poo {
    pub fn all(conn: &PgConnection) -> Vec<Poo> {
        all_poos
            .order(poo::published_at.desc())
            .load::<Poo>(conn)
            .expect("Failed to read data from DB")
    }
}
