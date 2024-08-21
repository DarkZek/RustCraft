#!/bin/sh

cd ./assets/textures

zip -r resources.pack ./game
mv ./resources.pack ../

cd ../..

git add ./assets/textures/resources.pack