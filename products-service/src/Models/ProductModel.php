<?php
namespace App\Models;

class ProductModel {
    private $productsFile;
    private $lastIdFile;

    public function __construct($filePath) {
        $this->productsFile = $filePath;
        $this->lastIdFile = dirname($filePath) . '/last_id.txt';
    }

    public function loadProducts() {
        if (file_exists($this->productsFile)) {
            $data = file_get_contents($this->productsFile);
            $products = json_decode($data, true);

            return is_array($products) ? $products : [];
        }

        file_put_contents($this->productsFile, json_encode([], JSON_PRETTY_PRINT));

        return [];
    }

    public function saveProducts($products) {
        if (!is_array($products)) {
            $products = [];
        }

        return file_put_contents($this->productsFile, json_encode($products, JSON_PRETTY_PRINT));
    }

    public function getProductById($id) {
    $products = $this->loadProducts();
    $id = intval($id);
    
    foreach ($products as $product) {
        if ($product['id'] === $id) {
            return $product;
        }
    }
    
    return null;
}

    public function getNextId() {
        if (file_exists($this->lastIdFile)) {
            $lastId = (int) file_get_contents($this->lastIdFile);
            $newId = $lastId + 1;

        } else {
            $newId = 1;
        }

        file_put_contents($this->lastIdFile, $newId);
        return $newId;
    }
    public function updateProduct($id, $data) {
        $products = $this->loadProducts();
        $id = intval($id);
        $found = false;
        
        foreach ($products as &$product) {
            if ($product['id'] === $id) {
                if (isset($data['name'])) {
                    $product['name'] = trim($data['name']);
                }

                if (isset($data['price'])) {
                    $product['price'] = floatval($data['price']);
                }

                if (isset($data['stock'])) {
                    $product['stock'] = intval($data['stock']);
                }

                $product['updated_at'] = date('Y-m-d H:i:s');
                $found = true;
                break;
            }
        }
        
        if ($found) {
            $this->saveProducts($products);

            return $products[array_search($id, array_column($products, 'id'))];
        }
        
        return null;
    }

    public function deleteProduct($id) {
        $products = $this->loadProducts();
        $id = intval($id);
        $initialCount = count($products);
        
        $products = array_filter($products, function($product) use ($id) {
            return $product['id'] !== $id;
        });
        
        $products = array_values($products);
        
        if (count($products) < $initialCount) {
            $this->saveProducts($products);
            return true;
        }
        
        return false;
    }
}
?>