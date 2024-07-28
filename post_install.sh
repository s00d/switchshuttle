#!/bin/bash

sudo chmod +x /Applications/switch-shuttle.app
sudo xattr -cr /Applications/switch-shuttle.app
sudo codesign --force --deep --sign - /Applications/switch-shuttle.app