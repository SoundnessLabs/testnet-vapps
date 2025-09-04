#!/usr/bin/env bash

set -e

echo "🚀 Installing soundnessup..." && echo

BASE_DIR=$HOME
SOUNDNESS_DIR=${SOUNDNESS_DIR-"$BASE_DIR/.soundness"}
SOUNDNESS_BIN_DIR="$SOUNDNESS_DIR/bin"
BIN_URL="https://raw.githubusercontent.com/soundnesslabs/soundness-layer/main/soundnessup/soundnessup"
BIN_PATH="$SOUNDNESS_BIN_DIR/soundnessup"

# Create the .soundness bin directory and soundnessup binary if it doesn't exist.
mkdir -p $SOUNDNESS_BIN_DIR
curl -# -L $BIN_URL -o $BIN_PATH
chmod +x $BIN_PATH

# Store the correct profile file (i.e. .profile for bash or .zshenv for ZSH).
case $SHELL in
  */zsh)
    PROFILE=${ZDOTDIR-"$HOME"}/.zshenv
    PREF_SHELL=zsh
    ;;
  */bash)
    PROFILE=$HOME/.bashrc
    PREF_SHELL=bash
    ;;
  */fish)
    PROFILE=$HOME/.config/fish/config.fish
    PREF_SHELL=fish
    ;;
  */ash)
    PROFILE=$HOME/.profile
    PREF_SHELL=ash
    ;;
  *)
    echo "soundnessup: could not detect shell, manually add ${SOUNDNESS_BIN_DIR} to your PATH."
    exit 1
esac

# Only add soundnessup if it isn't already in PATH.
if [[ ":$PATH:" != *":${SOUNDNESS_BIN_DIR}:"* ]]; then
  # Add the soundnessup directory to the path and ensure the old PATH variables remain.
  echo >> $PROFILE && echo "export PATH=\"\$PATH:$SOUNDNESS_BIN_DIR\"" >> $PROFILE
fi

echo && echo "✅ Installation complete!"
echo && echo "🔍 Detected shell: ${PREF_SHELL}"
echo "🔗 Added soundnessup to PATH"
echo && echo "To start using soundnessup, please run:"
echo && echo "▶ source ${PROFILE}"
echo "▶ soundnessup"
echo && echo "🎉 Enjoy using soundnessup! For help, type 'soundnessup --help'" 