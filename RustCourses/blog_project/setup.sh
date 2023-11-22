#!/bin/bash

# Paso 1: Instalar diesel-cli y las dependencias necesarias
echo "Instalando dependencias..."
sudo apt-get install python3-dev default-libmysqlclient-dev build-essential

echo "Instalando diesel-cli y dependencias..."
cargo install diesel_cli --no-default-features --features "mysql ssl"

# Paso 2: Configurar la base de datos
echo "Configurando la base de datos..."
# Reemplaza 'nombre_de_tu_proyecto' con el nombre de tu proyecto
diesel setup

# Paso 4: Modificar la migración con las tablas deseadas
# Abre el archivo generado en la carpeta 'migrations' y define las tablas según tus necesidades

# Paso 5: Aplicar la migración
echo "Aplicando migración..."
diesel migration run

# Opcional: Si necesitas deshacer una migración
# diesel migration revert

# Paso 6: ¡Listo!
echo "Configuración completa. ¡Tu proyecto está listo para usar Diesel con MySQL!"
