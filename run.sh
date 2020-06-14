export RUST_LOG_STYLE=always
export RUST_LOG=debug
export THT_IP_ADDRESS=0.0.0.0
export THT_PORT=19996
export THT_DEPLOYMENT_MODE=dev

cd ui
./generate_types.sh
npm ci --only=production
npm run build
cd ..
rm -rf templates/static
cp ui/dist/index.html templates/play.html
cp -R ui/dist/static templates/static
cargo run
