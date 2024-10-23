use core::panic;

use diesel::result::Error;
use diesel::{Connection, PgConnection};

use crud_bd::crud::{user::*, *};

const DEFAULT_USERNAME: &str = "Petrus";
const DEFAULT_PASSWORD: &str = "Petrus";

fn create_common_user(conn: &mut PgConnection) -> User {
    create_user(conn, DEFAULT_USERNAME, DEFAULT_PASSWORD).unwrap()
}

fn is_common_user(user: &User) {
    assert!(user.username.contains(DEFAULT_USERNAME));
    assert!(user.password.contains(DEFAULT_PASSWORD));
}

#[test]
fn user_created() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let user = create_common_user(conn);

        is_common_user(&user);

        Ok(())
    })
}

#[test]
fn find_user() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let created_user = create_common_user(conn);

        let found_user_id = get_user_by_id(conn, created_user.id).unwrap();
        is_common_user(&found_user_id);

        let found_username = get_user_by_username(conn, &created_user.username).unwrap();
        is_common_user(&found_username);

        Ok(())
    })
}

#[test]
fn modify_user() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let created_user = create_common_user(conn);

        let alt_username: &str = "zoquinha";
        let modified_user_username =
            update_user_username(conn, created_user.id, alt_username).unwrap();
        assert!(modified_user_username.username.contains(alt_username));

        let alt_password: &str = "kokichi123";
        let modified_user_password =
            update_user_password(conn, created_user.id, alt_password).unwrap();
        assert!(modified_user_password.password.contains(alt_password));

        Ok(())
    })
}

#[test]
fn remove_user() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let created_user = create_common_user(conn);

        let removed_user = delete_user(conn, created_user.id).unwrap();
        is_common_user(&removed_user);

        match get_user_by_id(conn, removed_user.id) {
            Ok(user) => panic!("User was found {:?}", user),
            Err(diesel::result::Error::NotFound) => Ok(()),
            Err(err) => panic!("{err}"),
        }
    })
}
