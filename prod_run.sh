#!/bin/sh

# Copy static into the bindmounted static directory
echo "Copying static into bindmounted static directory"
cp -R templates/static/* static/

# Run the binary
./target/release/team_heist_tactics
