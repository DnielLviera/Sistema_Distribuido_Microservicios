iniciar auth-service
uvicorn main:app --reload --port 8000

iniciar products-service
php -S localhost:8001

iniciar inventory-service
cargo run