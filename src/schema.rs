table! {
    department (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        create_date -> Nullable<Datetime>,
        last_updated -> Nullable<Datetime>,
    }
}

table! {
    student (id) {
        id -> Integer,
        number -> Nullable<Char>,
        name -> Nullable<Char>,
        department -> Nullable<Integer>,
        create_date -> Nullable<Datetime>,
        last_updated -> Nullable<Datetime>,
    }
}

joinable!(student -> department (department));

allow_tables_to_appear_in_same_query!(department, student,);
