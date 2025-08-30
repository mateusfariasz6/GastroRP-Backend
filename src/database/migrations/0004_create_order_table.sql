CREATE TABLE orders_table
(
    id          SERIAL PRIMARY KEY,
    total_price DOUBLE PRECISION NOT NULL,
    created_at  TIMESTAMP      NOT NULL DEFAULT NOW()
);