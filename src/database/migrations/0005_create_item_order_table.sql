CREATE TABLE order_items
(
    id         SERIAL PRIMARY KEY,
    order_id   INT NOT NULL REFERENCES orders_table (id) ON DELETE CASCADE,
    product_id INT NOT NULL REFERENCES products_table (id) ON DELETE CASCADE,
    quantity   INT NOT NULL
);
