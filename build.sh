docker run -v "$PWD":/volume --rm -t -e DATABASE_URL=mysql://root:passwd@172.17.0.1:3306/trish  clux/muslrust cargo build --release
docker build -t blendman974/trish .
docker save blendman974/trish | gzip | ssh aspaku 'cat | gzip -d | sudo docker load'