<?php
require_once __DIR__ . '/vendor/autoload.php';

use Psr\Http\Message\ResponseInterface as Response;
use Psr\Http\Message\ServerRequestInterface as Request;
use Slim\Factory\AppFactory;

use App\Models\ProductModel;
use App\Controllers\ProductController;
use App\Middleware\AuthMiddleware;

$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();

$app = AppFactory::create();
$app->addBodyParsingMiddleware();

$productModel = new ProductModel(__DIR__ . '/products.json');
$productController = new ProductController($productModel);
$authMiddleware = new AuthMiddleware($_ENV['SECRET_KEY']);

// Rutas
$app->get('/', function (Request $request, Response $response) {
    $response->getBody()->write(json_encode([
        'message' => 'Products Service funcionando'
    ]));

    return $response->withHeader('Content-Type', 'application/json');
});

// GET - Obtener todos los productos (Público)
$app->get('/products', function (Request $request, Response $response) use ($productController) {
    return $productController->getAllProducts($request, $response);
});

// GET - Obtener un producto por ID (Público)
$app->get('/products/{id}', function (Request $request, Response $response, $args) use ($productController) {
    return $productController->getProductById($request, $response, $args);
});

// POST - Crear producto (PROTEGIDO)
$app->post('/products', function (Request $request, Response $response) use ($productController) {
    return $productController->createProduct($request, $response);
})->add($authMiddleware);

// PUT - Actualizar producto (protegido)
$app->put('/products/{id}', function (Request $request, Response $response, $args) use ($productController) {
    return $productController->updateProduct($request, $response, $args);
})->add($authMiddleware);

// DELETE - Eliminar producto (protegido)
$app->delete('/products/{id}', function (Request $request, Response $response, $args) use ($productController) {
    return $productController->deleteProduct($request, $response, $args);
})->add($authMiddleware);




$app->get('/favicon.ico', function (Request $request, Response $response) {
    return $response->withStatus(204);
});

$app->run();
?>