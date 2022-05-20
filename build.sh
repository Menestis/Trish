docker run -it -v "$PWD":/volume --rm -t --network tmp clux/muslrust cargo build --release
# -e DATABASE_URL=mysql://root:passwd@172.17.0.1:3306/trish
cp target/x86_64-unknown-linux-musl/release/trish out/trish
docker build -t registry.aspaku.com/discord/trish -f Dockerfile --no-cache out/
docker push registry.aspaku.com/discord/trish
#docker save blendman974/trish | gzip | ssh helios.aspaku.com 'cat | gzip -d | sudo docker load'