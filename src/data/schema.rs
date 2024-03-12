table! {
    accounts (id) {
        id -> Integer,
        username -> Text,
        displayname -> Text,
        xp -> Integer,
        created_at -> Timestamp,
        current_avatar -> Nullable<Text>,
    }
}

table! {
    config (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    friendship_lookup (account_id_1, account_id_2) {
        account_id_1 -> Integer,
        account_id_2 -> Integer,
        friendship_id -> Integer,
    }
}

table! {
    friendships (id) {
        id -> Integer,
        account_id_1 -> Integer,
        account_id_2 -> Integer,
        favorite_1 -> Bool,
        favorite_2 -> Bool,
        initiated_by_1 -> Bool,
        initiated_at -> Timestamp,
        accepted_at -> Nullable<Timestamp>,
    }
}

table! {
    json_data (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    platform_id_links (id) {
        id -> Integer,
        platform -> Text,
        platform_id -> Text,
        account_id -> Integer,
    }
}

joinable!(friendship_lookup -> friendships (friendship_id));
joinable!(platform_id_links -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    config,
    friendship_lookup,
    friendships,
    json_data,
    platform_id_links,
);
