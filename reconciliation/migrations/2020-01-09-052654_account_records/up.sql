-- Your SQL goes here
CREATE TABLE account_records
(
    id                BIGINT PRIMARY KEY,
    serial_number     VARCHAR(255),
    biz_serial_number VARCHAR(255),
    tx_code           VARCHAR(255),
    account_id        VARCHAR(255),
    account_type      VARCHAR(255),
    currency          VARCHAR(255),
    `desc`            VARCHAR(255),
    amount            VARCHAR(255),
    balance           VARCHAR(255),
    available_balance VARCHAR(255),
    frozen_balance    VARCHAR(255),
    type              SMALLINT,
    timestamp         BIGINT,
    use_id            VARCHAR(255),
    user_type         SMALLINT,
    tx_type           VARCHAR(255),
    biz_fee           VARCHAR(255),
    rate              DECIMAL(20, 8),
    rate_currency     VARCHAR(255),
    rate_amount       VARCHAR(255)
)