table! {
    poo (id) {
        id -> Int4,
        form -> Int4,
        color -> Int4,
        bleeding -> Int4,
        required_time -> Time,
        published_at -> Timestamp,
    }
}

table! {
    poo_bleeding (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    poo_color (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    poo_form (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(poo -> poo_bleeding (bleeding));
joinable!(poo -> poo_color (color));
joinable!(poo -> poo_form (form));

allow_tables_to_appear_in_same_query!(poo, poo_bleeding, poo_color, poo_form,);
