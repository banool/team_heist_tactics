#!/bin/sh

# Copy static into the bindmounted static directory
echo "Copying static into bindmounted static directory"
cp -R templates/static/* static/

# Run the binary
./team_heist_tactics
