table! {
    account_records (id) {
        id -> Bigint,
        serial_number -> Nullable<Varchar>,
        biz_serial_number -> Nullable<Varchar>,
        tx_code -> Nullable<Varchar>,
        account_id -> Nullable<Varchar>,
        account_type -> Nullable<Varchar>,
        currency -> Nullable<Varchar>,
        desc -> Nullable<Varchar>,
        amount -> Nullable<Varchar>,
        balance -> Nullable<Varchar>,
        available_balance -> Nullable<Varchar>,
        frozen_balance -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Smallint>,
        timestamp -> Nullable<Bigint>,
        use_id -> Nullable<Varchar>,
        user_type -> Nullable<Smallint>,
        tx_type -> Nullable<Varchar>,
        biz_fee -> Nullable<Varchar>,
        rate -> Nullable<Decimal>,
        rate_currency -> Nullable<Varchar>,
        rate_amount -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    account_records,
    posts,
);
