use crud_bd::crud::{establish_connection, user::User};
use diesel::{PgConnection, QueryResult};

pub struct Database {
    pg_conn: PgConnection,
}

impl Database {
    pub fn new() -> Database {
        Database {
            pg_conn: establish_connection(),
        }
    }

    pub fn create_user(&mut self, user_username: &str, user_password: &str) -> QueryResult<User> {
        crud_bd::crud::user::create_user(&mut self.pg_conn, user_username, user_password)
    }

    pub fn delete_user(&mut self, user_id: i32) -> QueryResult<User> {
        crud_bd::crud::user::delete_user(&mut self.pg_conn, user_id)
    }

    pub fn update_username(&mut self, user_id: i32, new_username: &str) -> QueryResult<User> {
        crud_bd::crud::user::update_user_username(&mut self.pg_conn, user_id, new_username)
    }

    pub fn update_password(&mut self, user_id: i32, new_password: &str) -> QueryResult<User> {
        crud_bd::crud::user::update_user_password(&mut self.pg_conn, user_id, new_password)
    }

    pub fn fetch_users(
        &mut self,
        limit: i64,
        cursor: i64,
        filter: Option<String>,
    ) -> QueryResult<Vec<User>> {
        crud_bd::crud::user::get_users_with_pagination(&mut self.pg_conn, cursor, limit, filter)
    }

    // TODO: Remove cursor changes from here change the cursor
    pub fn next_users_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        filter: Option<String>,
    ) -> QueryResult<Vec<User>> {
        *db_cursor = db_cursor.saturating_add(limit);

        let users = self.fetch_users(limit, *db_cursor, filter)?;

        *db_cursor = *db_cursor + users.len() as i64 - limit;

        Ok(users)
    }

    pub fn prev_users_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        filter: Option<String>,
    ) -> QueryResult<Vec<User>> {
        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor -= limit;
        }

        let users = self.fetch_users(limit, *db_cursor, filter)?;

        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor = *db_cursor - users.len() as i64 + limit;
        }
        Ok(users)
    }
}
