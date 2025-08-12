create table account (
    id serial primary key,
    name text not null unique,
    password text,
    registered boolean default false,
    created timestamp default now(),
    updated timestamp default now(),
    email text not null,
    x jsonb
);
create index account_name_idx on account(name);

create table session(
    id text not null,
    account_id integer references account(id),
    created timestamp default now(),
    updated timestamp,
    primary key(id, account_id)
);
create index session_id on session(id);
create index session_account on session(account_id);

create table channel (
    id serial primary key,
    parent_id integer,
    name text not null
);
create index channel_name_idx on channel(name);

create table channel_account (
    channel_id integer references channel (id),
    account_id integer references account (id),
    owner boolean default false,
    unique(channel_id, account_id)
);
create index channel_account_c on channel_account(channel_id);
create index channel_account_a on channel_account(account_id);

create table message (
    channel_id integer references channel (id),
    account_id integer references account (id),
    created timestamp default now(),
    content text not null
);
create index message_c on message(channel_id);
create index message_a on message(account_id);
