// @generated automatically by Diesel CLI.

diesel::table! {
    students (id) {
        id -> Int4,
        name -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        course -> Varchar,
    }
}
