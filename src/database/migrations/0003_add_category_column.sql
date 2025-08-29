CREATE TYPE product_category AS ENUM ('DRINK', 'FOOD', 'UNDEFINED');

ALTER TABLE products_table
    ADD COLUMN category product_category NOT NULL DEFAULT 'UNDEFINED';
