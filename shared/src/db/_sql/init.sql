PRAGMA foreign_keys = ON;

create table backoffice_users
(
    id       integer primary key autoincrement not null,
    username text unique                       not null,
    password blob                              not null,
    role     text                              not null
);

create table user_login_tokens
(
    user_id      integer     not null,
    token        text unique not null,
    expire_after text        not null,
    foreign key (user_id) references backoffice_users (id) on delete cascade
);

create table error_stack
(
    id            integer primary key autoincrement not null,
    error_name    text                              not null,
    error_summary text                              not null,
    error_stack   text                              not null,
    reported_at   text                              not null
);

create table cms_page
(
    id      integer primary key autoincrement not null,
    user_id integer                           not null,
    title   text                              not null,
    summary text                              not null,
    added   text                              not null default CURRENT_TIMESTAMP,
    updated text,
    status  text                              not null,
    foreign key (user_id) references backoffice_users (id) on delete cascade
);

create table cms_component
(
    id        integer primary key autoincrement not null,
    page_id   integer                           not null,
    kind_uuid text                              not null,
    position  integer                           not null,
    label     text                              not null,
    raw_data  blob                              not null,
    foreign key (page_id) references cms_page (id) on delete cascade
);