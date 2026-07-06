use diesel::PgConnection;
use diesel::prelude::*;

use crate::db::{
    models::{NewUser, UserRow},
    schema::users,
};

pub fn find_by_email(conn: &mut PgConnection, target_email: &str) -> QueryResult<Option<UserRow>> {
    users::table
        .filter(users::email.eq(target_email))
        .select(UserRow::as_select())
        .first(conn)
        .optional()
}

pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<UserRow> {
    diesel::insert_into(users::table)
        .values(new_user)
        .returning(UserRow::as_returning())
        .get_result(conn)
}