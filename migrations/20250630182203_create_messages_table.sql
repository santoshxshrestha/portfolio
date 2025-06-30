-- Add migration script here
CREATE TABLE blog (
id SERIAL PRIMARY KEY not null,
title TEXT not null,
views int not null default 0,
content TEXT not null,
excerpt TEXT not null,
created_at TIMESTAMP DEFAULT (current_timestamp at time zone 'utc' at time zone 'Asia/Kathmandu') not null
);
