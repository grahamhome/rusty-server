create table task (
    id integer not null,
    title text not null unique,
    done integer not null default 0,
    primary key (id)
);