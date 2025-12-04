use sqlx::PgPool;

pub async fn init_db(pool: &PgPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            product_id SERIAL PRIMARY KEY,
            product_name VARCHAR(255) NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(pool)
    .await
    .expect("Fallo al crear la tabla");
    
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(pool)
        .await
        .expect("Fallo al contar productos");
    
    if count.0 == 0 {
        sqlx::query(
            r#"
            INSERT INTO products (product_name, stock) VALUES
            ('Laptop', 10),
            ('Mouse', 50),
            ('Teclado', 30),
            ('Monitor', 15)
            "#
        )
        .execute(pool)
        .await
        .expect("Fallo al insertar datos de ejemplo");
    }
}