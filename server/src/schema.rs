// @generated automatically by Diesel CLI.

diesel::table! {
    scores (id) {
        id -> Int4,
        #[max_length = 24]
        name -> Varchar,
        time -> Interval,
    }
}
