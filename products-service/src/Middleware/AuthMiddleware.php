<?php
namespace App\Middleware;

use Firebase\JWT\JWT;
use Firebase\JWT\Key;

class AuthMiddleware {
    private $secretKey;

    public function __construct($secretKey) {
        $this->secretKey = $secretKey;
    }

    public function __invoke($request, $handler) {
        $token = null;
        $authHeader = $request->getHeaderLine('Authorization');

        if (preg_match('/bearer\s+(.*)$/i', $authHeader, $matches)) {
            $token = $matches[1];
        }

        if (!$token) {
            $response = new \Slim\Psr7\Response();
            $response->getBody() -> write(json_encode([
                'error' => 'Token requerido'
            ]));
            
            return $response->withStatus(401)->withHeader('Content-Type', 'application/json');
        }

        try {
            $decoded = JWT::decode($token, new Key($this->secretKey, 'HS256'));
            $request = $request->withAttribute('user', $decoded);

        } catch (\Exception $e) {
            $response = new \Slim\Psr7\Response();
            $response->getBody()->write(json_encode([
                'error' => 'Token inválido'
            ]));
            
            return $response->withStatus(401)->withHeader('Content-Type', 'application/json');
        }

        return $handler->handle($request);
    }
}
?>