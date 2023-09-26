!bin/zsh
. .venv/bin/activate
rye build --wheel
rye publish dist/pysvb-0.2.0-cp311-cp311-macosx_11_0_arm64.whl