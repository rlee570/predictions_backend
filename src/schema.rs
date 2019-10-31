table! {
    predictions (id) {
        id -> Int4,
        owner -> Int4,
        statement -> Varchar,
        expiry -> Timestamp,
        outcome -> Bool,
        votes -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        points -> Int4,
        role -> Text,
        hash -> Text,
    }
}

table! {
    votes (id) {
        id -> Int4,
        prediction -> Int4,
        user_id -> Int4,
        points -> Int4,
    }
}

joinable!(predictions -> users (owner));
joinable!(votes -> predictions (prediction));
joinable!(votes -> users (user_id));

allow_tables_to_appear_in_same_query!(predictions, users, votes,);
