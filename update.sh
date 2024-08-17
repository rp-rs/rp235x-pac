#!/usr/bin/env bash

# Path to `svd`/`svdtools`
SVDTOOLS="${SVDTOOLS:-svdtools}"
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

set -exuo pipefail

cargo install --version 0.33.4 svd2rust
cargo install --version 0.12.1  form
rustup component add rustfmt
if [ "$SVDTOOLS" == "svdtools" ]; then
    cargo install --version 0.3.17 svdtools
else
    python3 -mvenv --clear .venv
    source .venv/bin/activate
    pip3 install --upgrade "svdtools==0.1.25"
fi

$SVDTOOLS patch svd/RP2350.yaml

if [ "$SVDTOOLS" != "svdtools" ]; then
    deactivate
fi


# Most of the code is from Cortex-M mode
tmp_dir=$(mktemp -d -t svd2rust-XXXX)
pushd ${tmp_dir}
svd2rust -i ${SCRIPT_DIR}/svd/RP2350.svd.patched -c ${SCRIPT_DIR}/svd2rust.toml --target cortex-m
form -i mod.rs -o inner
rustfmt inner/lib.rs
mv inner/lib.rs inner/mod_cortex_m.rs
rm -rf ${SCRIPT_DIR}/src/inner
mv inner ${SCRIPT_DIR}/src
popd
rm -rf ${tmp_dir}

# But RISC-V mode needs a custom mod.rs
tmp_dir=$(mktemp -d -t svd2rust-XXXX)
pushd ${tmp_dir}
svd2rust -i ${SCRIPT_DIR}/svd/RP2350.svd.patched -c ${SCRIPT_DIR}/svd2rust.toml --target riscv
form -i mod.rs -o inner
rustfmt inner/lib.rs
mv inner/lib.rs ${SCRIPT_DIR}/src/inner/mod_risc_v.rs
# This module isn't in the Cortex-M version - everything else is
mv inner/interrupt* ${SCRIPT_DIR}/src/inner
popd
rm -rf ${tmp_dir}

cargo fmt

# Original svd has \n (two chars) in it, which gets converted to "\n" by svd2rust
# If we convert them to newline characters in the SVD, they don't turn up in markdown so docs suffers
# So, convert \n to [spc] [spc] [newline], then strip the spaces out if there are consecutive [newlines]
# This means that by the time we're in markdown \n\n becomes new paragraph, and \n becomes a new line
if [ "$(uname)" == "Darwin" ]; then
    find src -name '*.rs' -exec sed -i '' -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
else
    find src -name '*.rs' -exec sed -i -e 's/\\n/  \n/g' -e 's/\n  \n/\n\n/g' {} \;
fi

# Sort specified fields alphanumerically for easier consumption in docs.rs
./sortFieldsAlphaNum.sh src/inner/mod_cortex_m.rs
./sortFieldsAlphaNum.sh src/inner/mod_risc_v.rs
