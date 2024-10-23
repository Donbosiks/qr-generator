#!/bin/bash
pacman -Sy
yes "" | pacman -S mariadb-lts
mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql
systemctl enable mysqld
systemctl start mysqld
sudo mariadb -u root -e "CREATE USER 'qr_user'@'localhost' IDENTIFIED BY 'qr_project';"
sudo mariadb -u root -e "GRANT ALL PRIVILEGES ON *.* TO 'qr_user'@'localhost' WITH GRANT OPTION;"
mariadb -u qr_user -pqr_project -e "CREATE DATABASE Qr_main_db;"
echo DATABASE_URL="mysql://qr_user:qr_project@localhost/Qr_main_db" > .env
yes "" | pacman -S rust
yes "" | pacman -S postgresql-libs
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
cargo build
diesel migration run
rm ./src/schema.rs
yes "" | pacman -S wget
wget -O ./src/schema.rs https://raw.githubusercontent.com/Donbosiks/qr-generator/master/src/schema.rs

