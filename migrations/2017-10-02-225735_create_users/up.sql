CREATE TABLE coffeezera_users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  telegram_id BIGINT NOT NULL,
  account_balance FLOAT NOT NULL
);