use crate::schema;
use diesel::prelude::*;

use self::schema::users::dsl::*;
use crate::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

pub fn create_user<'a>(
    conn: &mut PgConnection,
    user_username: &'a str,
    pswd: &'a str,
) -> QueryResult<User> {
    let new_user = NewUser {
        username: user_username,
        password: pswd,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
    users.find(user_id).first(conn)
}

// maybe not neeeded anymore
pub fn get_user_by_username(conn: &mut PgConnection, user_username: &str) -> QueryResult<User> {
    users.filter(username.ilike(user_username)).first(conn)
}

pub fn user_authenticate(
    conn: &mut PgConnection,
    user_username: &str,
    user_password: &str,
) -> QueryResult<bool> {
    let user = get_user_by_username(conn, user_username)?;
    Ok(user.password == user_password)
}
// general purpose function
pub fn get_users_with_pagination(
    conn: &mut PgConnection,
    offset: i64,
    limit: i64,
    filter_username: Option<String>,
    filter_id: Option<i32>,
) -> QueryResult<Vec<User>> {
    let mut query = users.into_boxed();

    if let Some(filter_username) = filter_username {
        let filter_username: String = format!("%{filter_username}%");
        query = query.filter(username.ilike(filter_username));
    }

    if let Some(filter_id) = filter_id {
        query = query.filter(id.eq(filter_id));
    }

    query.offset(offset).limit(limit).load::<User>(conn)
}

pub fn update_user_password(
    conn: &mut PgConnection,
    user_id: i32,
    new_password: &str,
) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set(password.eq(new_password))
        .get_result(conn)
}

pub fn update_user_username(
    conn: &mut PgConnection,
    user_id: i32,
    new_username: &str,
) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set(username.eq(new_username))
        .get_result(conn)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
    diesel::delete(users.find(user_id)).get_result(conn)
}
