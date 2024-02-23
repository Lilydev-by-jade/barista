-- Add migration script here

CREATE TABLE IF NOT EXISTS triage (
    guild_id bigint PRIMARY KEY,
    enabled bool NOT NULL DEFAULT false,
    moderator_channel_id bigint,
    member_role_id bigint
)
