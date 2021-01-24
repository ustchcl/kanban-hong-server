table! {
    account (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        icon -> Nullable<Mediumtext>,
        create_time -> Timestamp,
    }
}

table! {
    milestone (id) {
        id -> Integer,
        title -> Varchar,
        description -> Mediumtext,
        deadline -> Timestamp,
        project_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Mediumtext,
        published -> Bool,
    }
}

table! {
    project (id) {
        id -> Integer,
        categories -> Mediumtext,
        description -> Mediumtext,
        title -> Mediumtext,
        creator_id -> Integer,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

table! {
    project_account_relation (id) {
        id -> Integer,
        project_id -> Integer,
        account_id -> Integer,
    }
}

table! {
    tag (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    task (id) {
        id -> Integer,
        title -> Varchar,
        description -> Mediumtext,
        parent_id -> Nullable<Integer>,
        category -> Varchar,
        creator_id -> Integer,
        milestone_id -> Nullable<Integer>,
    }
}

table! {
    task_account_relation (id) {
        id -> Integer,
        task_id -> Integer,
        account_id -> Integer,
    }
}

table! {
    task_tag_relation (id) {
        id -> Integer,
        task_id -> Integer,
        tag_id -> Integer,
    }
}

joinable!(milestone -> project (project_id));
joinable!(project -> account (creator_id));
joinable!(project_account_relation -> account (account_id));
joinable!(project_account_relation -> project (project_id));
joinable!(task -> account (creator_id));
joinable!(task -> milestone (milestone_id));
joinable!(task_account_relation -> account (account_id));
joinable!(task_account_relation -> task (task_id));
joinable!(task_tag_relation -> tag (tag_id));
joinable!(task_tag_relation -> task (task_id));

allow_tables_to_appear_in_same_query!(
    account,
    milestone,
    posts,
    project,
    project_account_relation,
    tag,
    task,
    task_account_relation,
    task_tag_relation,
);
