use diesel::prelude::{Insertable, Queryable, Selectable};
use uuid::Uuid;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub user_key: String,
    pub salt: String,
    pub vault: String,
    pub iterations: i32,
    pub vaultiv: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub user_key: String,
    pub salt: String,
    pub vault: String,
    pub iterations: i32,
    pub vaultiv: String,
}