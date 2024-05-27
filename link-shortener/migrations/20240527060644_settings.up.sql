create table if not exists settings
(
    id          text default 'DEFAULT_SETTINGS' not null primary key,
    encrypted_global_api_key text not null
);

insert into settings (id, encrypted_global_api_key) values ('DEFAULT_SETTINGS', 'c50770e5e7bbfa25be465730e487d50bfd95eb3c5e0db23e96663e4958306acc');