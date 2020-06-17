export RUST_LOG_STYLE=always
export RUST_LOG=debug
export THT_IP_ADDRESS=0.0.0.0
export THT_PORT=19996

DEV_MODE=dev

export THT_DEPLOYMENT_MODE=$DEV_MODE

echo "Doing $DEV_MODE build"

cd ui
rm -rf dist
cp ../src/types.proto ui
./generate_types.sh
rm types.proto
if [ "$DEV_MODE" = "dev" ]; then
    echo 'Note: Not running npm install'
    npm run devbuild
elif [ "$DEV_MODE" = "prod" ]; then
    npm ci --only=production
    npm run prodbuild
else 
    echo 'Invalid dev mode';
    exit;
fi
cd ..
cp ui/dist/index.html templates/play.html
cp -R ui/dist/static/* static/
cargo run
