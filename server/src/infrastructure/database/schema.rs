// @generated automatically by Diesel CLI.

diesel::table! {
    brawlers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        display_name -> Varchar,
        #[max_length = 512]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_public_id -> Nullable<Varchar>,
        #[max_length = 512]
        cover_url -> Nullable<Varchar>,
        #[max_length = 255]
        cover_public_id -> Nullable<Varchar>,
        bio -> Nullable<Text>,
    }
}

diesel::table! {
    crew_memberships (mission_id, brawler_id) {
        mission_id -> Int4,
        brawler_id -> Int4,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    friendships (id) {
        id -> Int4,
        user_id -> Int4,
        friend_id -> Int4,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    mission_chat_messages (id) {
        id -> Int4,
        mission_id -> Int4,
        brawler_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
        image_url -> Nullable<Text>,
    }
}

diesel::table! {
    mission_invitations (id) {
        id -> Int4,
        mission_id -> Int4,
        inviter_id -> Int4,
        invitee_id -> Int4,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    mission_ratings (id) {
        id -> Int4,
        mission_id -> Int4,
        brawler_id -> Int4,
        rating -> Int4,
        comment -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    missions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        status -> Varchar,
        chief_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        #[max_length = 512]
        image_url -> Nullable<Varchar>,
        #[max_length = 5]
        code -> Varchar,
    }
}

diesel::joinable!(crew_memberships -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> missions (mission_id));
diesel::joinable!(mission_chat_messages -> brawlers (brawler_id));
diesel::joinable!(mission_chat_messages -> missions (mission_id));
diesel::joinable!(mission_invitations -> missions (mission_id));
diesel::joinable!(mission_ratings -> brawlers (brawler_id));
diesel::joinable!(mission_ratings -> missions (mission_id));
diesel::joinable!(missions -> brawlers (chief_id));

diesel::allow_tables_to_appear_in_same_query!(
    brawlers,
    crew_memberships,
    friendships,
    mission_chat_messages,
    mission_invitations,
    mission_ratings,
    missions,
);
