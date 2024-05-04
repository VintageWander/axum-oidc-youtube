-- Add up migration script here
create table oidc_state (
    csrf_token text primary key,
    code_verifier text not null,
    nonce text not null
);