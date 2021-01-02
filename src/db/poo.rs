use chrono::{NaiveDateTime, NaiveTime};
use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::poo::dsl::poo as all_poos;
use crate::schema::*;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Queryable)]
pub struct Poo {
    id: i32,
    pub form: String,
    pub color: String,
    pub bleeding: String,
    pub required_time: NaiveTime,
    pub published_at: NaiveDateTime,
}

#[table_name = "poo"]
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Insertable)]
pub struct RawPoo {
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
            .inner_join(poo_form::table)
            .inner_join(poo_color::table)
            .inner_join(poo_bleeding::table)
            .select((
                poo::id,
                poo_form::name,
                poo_color::name,
                poo_bleeding::name,
                poo::required_time,
                poo::published_at,
            ))
            .load(conn)
            .expect("Failed to read data from DB")
    }

    pub fn insert(conn: &PgConnection, form: RawPoo) -> bool {
        diesel::insert_into(poo::table)
            .values(form)
            .execute(conn)
            .is_ok()
    }
}
