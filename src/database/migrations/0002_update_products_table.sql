ALTER TABLE products_table
    ALTER COLUMN price TYPE DOUBLE PRECISION
    USING price::double precision;
