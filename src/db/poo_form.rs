use super::*;

use crate::schema::poo_form;
use crate::schema::poo_form::dsl::poo_form as all_poo_forms;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable)]
pub struct PooForm {
    pub id: i32,
    pub name: String,
}

impl PooForm {
    pub fn all(conn: &PgConnection) -> Vec<PooForm> {
        all_poo_forms
            .order(poo_form::id.asc())
            .load(conn)
            .expect("Failed to read data from DB")
    }
}
