use super::*;

use chrono::{NaiveDateTime, NaiveTime};

use crate::schema::poo::dsl::poo as all_poos;
use crate::schema::{poo, poo_bleeding, poo_color, poo_form};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Queryable)]
pub struct Poo {
    pub id: i32,
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

    pub fn delete_with_id(conn: &PgConnection, id: i32) -> bool {
        diesel::delete(all_poos.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &PgConnection) -> bool {
        diesel::delete(all_poos).execute(conn).is_ok()
    }
}
