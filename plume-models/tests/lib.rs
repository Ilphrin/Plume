extern crate diesel;
#[macro_use] extern crate diesel_migrations;

extern crate plume_models;

use diesel::Connection;
use plume_models::{
    Connection as Conn,
    instance::*,
    safe_string::SafeString,
};

embed_migrations!("../migrations/sqlite");

fn db() -> Conn {
    let conn = Conn::establish(":memory:").expect("Couldn't connect to the database");
    embedded_migrations::run(&conn).expect("Couldn't run migrations");
    conn
}

#[test]
fn instance_insert() {
    let conn = &db();
    Instance::insert(conn, NewInstance {
        default_license: "WTFPL".to_string(),
        local: true,
        long_description: SafeString::new("This is my instance."),
        long_description_html: "<p>This is my instance</p>".to_string(),
        short_description: SafeString::new("My instance."),
        short_description_html: "<p>My instance</p>".to_string(),
        name: "My instance".to_string(),
        open_registrations: true,
        public_domain: "plu.me".to_string(),
    });
    let inst = Instance::get_local(conn);
    assert!(inst.is_some());
    let inst = inst.unwrap();
    assert_eq!(inst.name, "My instance".to_string());
}
