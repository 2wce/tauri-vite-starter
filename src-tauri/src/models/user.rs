use diesel::prelude::*;
use crate::schema::users;

/// The model representing a row in the `users` database table.
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
  pub fn find(conn: &mut impl Conn, id: i32) -> QueryResult<User> {
      users::table.find(id).first(conn)
  }

  pub fn find_by_email(conn: &mut impl Conn, email: &str) -> QueryResult<User> {
      users::table
          .filter(lower(users::email).eq(email.to_lowercase()))
          .order(users::email.desc())
          .first(conn)
  }
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}