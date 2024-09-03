#!/bin/bash
pacman -Sy
yes "" | pacman -S mariadb-lts
mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql
systemctl enable mysqld
systemctl start mysqld
sudo mysql -u root -e "CREATE DATABASE Qr_main_db;"
echo DATABASE_URL=mysql://root@localhost/Qr_main_db > .env
yes "" | pacman -S rust
yes "" | pacman -S postgresql-libs
yes "" | pacman -S diesel-cli
cargo build
diesel migration run
rm ./src/schema.rs
yes "" | pacman -S wget
wget -O ./src/schema.rs https://raw.githubusercontent.com/Donbosiks/qr-generator/master/src/schema.rs

