export RUST_LOG_STYLE=always
export RUST_LOG=debug
export THT_IP_ADDRESS=0.0.0.0
export THT_PORT=19996
export HANDLES_FILE="data/handles.txt"

DEV_MODE=dev

export THT_DEPLOYMENT_MODE=$DEV_MODE

echo "Doing $DEV_MODE build"

cd ui
rm -rf dist
./generate_types.sh
echo 'Note: Not running yarn install'
if [ "$DEV_MODE" = "dev" ]; then
    yarn run devbuild
elif [ "$DEV_MODE" = "prod" ]; then
    echo 'Note: Instead of using prodbuild from this script, build the docker image'
    yarn run prodbuild
else 
    echo 'Invalid dev mode';
    exit;
fi
cd ..
cp ui/dist/index.html templates/play.html
cp ui/src/components/main.css static/main.css
cp -R ui/dist/static/* static/
cargo run
rm templates/play.html
