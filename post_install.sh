#!/bin/bash

sudo chmod +x /Users/s00d/Downloads/FemdomStories.app
sudo xattr -cr /Users/s00d/Downloads/FemdomStories.app
sudo codesign --force --deep --sign - /Users/s00d/Downloads/FemdomStories.app
