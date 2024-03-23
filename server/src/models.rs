use diesel::{data_types::PgInterval, prelude::*};
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::scores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Score {
    pub id: i32,
    pub name: String,
    pub time: PgInterval,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::scores)]
pub struct NewScore {
    pub name: String,
    pub time: PgInterval,
}
