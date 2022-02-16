#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use schema::foo::dsl::*;

mod schema {
    diesel::table! {
        foo {
            id -> diesel::sql_types::Uuid,
        }
    }
}

#[derive(Queryable)]
struct Foo {
    pub id: uuid::Uuid,
}

fn main() {
    let connection = PgConnection::establish("").unwrap();
    foo.load::<Foo>(&connection);

    println!("Hello, world!");
}
