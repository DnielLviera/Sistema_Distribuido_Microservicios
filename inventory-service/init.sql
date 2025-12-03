-- init.sql
CREATE TABLE IF NOT EXISTS products (
    product_id SERIAL PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL,
    stock INTEGER NOT NULL DEFAULT 0
);

-- Insertar datos de ejemplo basados en tu inventory.json
INSERT INTO products (product_id, product_name, stock) VALUES
(1, 'Laptop Gaming', 7),
(2, 'Mouse Inalámbrico', 42),
(3, 'Teclado Mecánico', 8),
(4, 'Monitor 24"', 45),
(5, 'Auriculares Bluetooth', 23)
ON CONFLICT (product_id) DO UPDATE SET
    product_name = EXCLUDED.product_name,
    stock = EXCLUDED.stock;