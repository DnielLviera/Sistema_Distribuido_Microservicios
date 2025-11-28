FROM php:8.2-apache

WORKDIR /var/www/html

# Habilitar mod_rewrite para Slim
RUN a2enmod rewrite

# Instalar extensiones necesarias
RUN apt-get update && apt-get install -y \
    libzip-dev \
    zip \
    unzip \
    && docker-php-ext-install zip

# Instalar Composer
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Copiar archivos
COPY . .

# Instalar dependencias
RUN composer install --no-dev --optimize-autoloader

# Configurar Apache para Slim
RUN echo '<VirtualHost *:80>\n\
    DocumentRoot /var/www/html\n\
    <Directory "/var/www/html">\n\
        AllowOverride All\n\
        Require all granted\n\
    </Directory>\n\
</VirtualHost>' > /etc/apache2/sites-available/000-default.conf

# Puerto
EXPOSE 8001

# Configurar Apache para puerto 8001
RUN echo 'Listen 8001' >> /etc/apache2/ports.conf
RUN sed -i 's/80/8001/g' /etc/apache2/sites-available/000-default.conf
RUN sed -i 's/80/8001/g' /etc/apache2/ports.conf

CMD ["apache2-foreground"]