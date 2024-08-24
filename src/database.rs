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
    // pub fn fetch_users(&mut self, limit: i64, cursor: i64, filter: Option<String>) -> Vec<User> {
    //     match crud_bd::crud::user::get_users_with_pagination(
    //         &mut self.pg_conn,
    //         cursor,
    //         limit,
    //         filter,
    //     ) {
    //         Ok(users) => {
    //             if users.is_empty() {
    //                 return 0;
    //             }
    //             users
    //         }
    //         Err(err) => {
    //             self.set_prompt_message(Some(Err(std::io::Error::new(
    //                 std::io::ErrorKind::Other,
    //                 format!("Failed to fetch user. {:?}", err.to_string()),
    //             ))));

    //             self.set_db_cursor(0);
    //             Vec::new()
    //         }
    //     };

    //     self.users.len()
    // }
}
