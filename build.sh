docker run -v "$PWD":/volume --rm -t --network tmp -e DATABASE_URL=mysql://root:passwd@172.17.0.1:3306/trish clux/muslrust cargo build --release
docker build -t registry.aspaku.com/discord/trish --no-cache .
docker push registry.aspaku.com/discord/trish
#docker save blendman974/trish | gzip | ssh helios.aspaku.com 'cat | gzip -d | sudo docker load'