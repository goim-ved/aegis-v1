-- Add migration script here
CREATE TABLE IF NOT EXISTS legal_entities (
    id SERIAL PRIMARY KEY,
    hash_id VARCHAR(255) NOT NULL UNIQUE,
    jurisdiction VARCHAR(100) NOT NULL,
    kyc_level SMALLINT NOT NULL CHECK (kyc_level >= 0),
    on_chain_id VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
