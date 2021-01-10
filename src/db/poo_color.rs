use super::*;

use crate::schema::poo_color;
use crate::schema::poo_color::dsl::poo_color as all_poo_colors;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable)]
pub struct PooColor {
    pub id: i32,
    pub name: String,
}

impl PooColor {
    pub fn all(conn: &PgConnection) -> Vec<PooColor> {
        all_poo_colors
            .order(poo_color::id.asc())
            .load(conn)
            .expect("Failed to read data from DB")
    }
}
