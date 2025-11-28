<?php
namespace App\Controllers;

use App\Models\ProductModel;

class ProductController {
    private $productModel;

    public function __construct($productModel) {
        $this->productModel = $productModel;
    }

    public function getAllProducts($request, $response) {
        $products = $this->productModel->loadProducts();
        $response->getBody()->write(json_encode($products));

        return $response->withHeader('Content-Type', 'application/json');
    }

    public function getProductById($request, $response, $args) {
    $id = $args['id'] ?? null;
    
    if (!$id || !is_numeric($id)) {
        $response->getBody()->write(json_encode([
            'error' => 'ID de producto debe ser un número válido'
        ]));

        return $response->withStatus(400);
    }
    
    $product = $this->productModel->getProductById($id);
    
    if ($product) {
        $response->getBody()->write(json_encode($product));

        return $response->withHeader('Content-Type', 'application/json');
    }
    
    $response->getBody()->write(json_encode([
        'error' => 'Producto no encontrado'
    ]));

    return $response->withStatus(404);
}

    public function createProduct($request, $response) {
        try {
            $data = $request->getParsedBody();
            $products = $this->productModel->loadProducts();
            
            if (empty($data['name']) || !isset($data['price']) || !isset($data['stock'])) {
                $response->getBody()->write(json_encode([
                    'error' => 'Datos incompletos. Se requieren: name, price, stock'
                ]));

                return $response->withStatus(400);
            }

            $newProduct = [
                'id' => $this->productModel->getNextId(),
                'name' => trim($data['name']),
                'price' => floatval($data['price']),
                'stock' => intval($data['stock']),
                'created_at' => date('Y-m-d H:i:s')
            ];
            
            $products[] = $newProduct;
            $this->productModel->saveProducts($products);
            
            $response->getBody()->write(json_encode([
                'message' => 'Producto creado exitosamente',
                'product' => $newProduct
            ]));

            return $response->withStatus(201);

        } catch (\Exception $e) {
            $response->getBody()->write(json_encode([
                'error' => 'Error interno del servidor: ' . $e->getMessage()
            ]));

            return $response->withStatus(500);
        }
    }

    public function updateProduct($request, $response, array $args) {
        try {
            $id = $args['id'] ?? null;
            $data = $request->getParsedBody();
            
            if (!$id || !is_numeric($id)) {
                $response->getBody()->write(json_encode([
                    'error' => 'ID debe ser un número válido'
                ]));

                return $response->withStatus(400);
            }
            
            if (empty($data) || (!isset($data['name']) && !isset($data['price']) && !isset($data['stock']))) {
                $response->getBody()->write(json_encode([
                    'error' => 'Se requiere al menos un campo para actualizar: name, price o stock'
                ]));

                return $response->withStatus(400);
            }
            
            $updatedProduct = $this->productModel->updateProduct($id, $data);
            
            if ($updatedProduct) {
                $response->getBody()->write(json_encode([
                    'message' => 'Producto actualizado exitosamente',
                    'product' => $updatedProduct
                ]));

                return $response->withHeader('Content-Type', 'application/json');
            }
            
            $response->getBody()->write(json_encode([
                'error' => "Producto con ID {$id} no encontrado"
            ]));

            return $response->withStatus(404);
            
        } catch (\Exception $e) {
            $response->getBody()->write(json_encode([
                'error' => 'Error interno del servidor: ' . $e->getMessage()
            ]));

            return $response->withStatus(500);
        }
    }

    public function deleteProduct($request, $response, array $args) {
        try {
            $id = $args['id'] ?? null;
            
            if (!$id || !is_numeric($id)) {
                $response->getBody()->write(json_encode([
                    'error' => 'ID debe ser un número válido'
                ]));

                return $response->withStatus(400);
            }
            
            $deleted = $this->productModel->deleteProduct($id);
            
            if ($deleted) {
                $response->getBody()->write(json_encode([
                    'message' => "Producto con ID {$id} eliminado exitosamente"
                ]));

                return $response->withHeader('Content-Type', 'application/json');
            }
            
            $response->getBody()->write(json_encode([
                'error' => "Producto con ID {$id} no encontrado"
            ]));

            return $response->withStatus(404);
            
        } catch (\Exception $e) {
            $response->getBody()->write(json_encode([
                'error' => 'Error interno del servidor: ' . $e->getMessage()
            ]));

            return $response->withStatus(500);
        }
    }
}
?>