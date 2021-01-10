use super::*;

use crate::schema::poo_bleeding;
use crate::schema::poo_bleeding::dsl::poo_bleeding as all_poo_bleedings;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable)]
pub struct PooBleeding {
    pub id: i32,
    pub name: String,
}

impl PooBleeding {
    pub fn all(conn: &PgConnection) -> Vec<PooBleeding> {
        all_poo_bleedings
            .order(poo_bleeding::id.asc())
            .load(conn)
            .expect("Failed to read data from DB")
    }
}
