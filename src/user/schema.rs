
table! {
    users (id) {
        id -> Nullable<Integer>,
        firstname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        registration_code -> Nullable<Varchar>,
        reset_code -> Nullable<Varchar>,
        image -> Nullable<Mediumblob>,
        create_date -> Unsigned<Bigint>,
        edit_date -> Unsigned<Bigint>,
    }
}