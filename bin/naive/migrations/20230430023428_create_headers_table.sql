-- Add migration script here
CREATE table headers (
    hash TEXT PRIMARY KEY NOT NULL,

    parent_hash TEXT NOT NULL,

    uncles_hash TEXT NOT NULL,

    author TEXT NOT NULL,

    state_root TEXT NOT NULL,

    transactions_root TEXT NOT NULL,

    receipts_root TEXT NOT NULL,

    number NUMERIC NOT NULL,

    gas_used NUMERIC NOT NULL,

    gas_limit NUMERIC NOT NULL,

    extra_data BYTEA NOT NULL,

    logs_bloom BYTEA NOT NULL,

    timestamp NUMERIC NOT NULL,

    difficulty TEXT NOT NULL,

    size NUMERIC NOT NULL,

    mix_hash TEXT NOT NULL,

    nonce TEXT NOT NULL,
    
    base_fee_per_gas NUMERIC
);
