#!/bin/bash

mkdir target_deploy

pushd libs/ffxiv_parser > /dev/null 2>&1
cargo build --no-default-features --target-dir ../../target_ffxiv_parser
popd > /dev/null 2>&1

pushd libs/sqpack_reader > /dev/null 2>&1
cargo build --no-default-features --target-dir ../../target_sqpack_reader
popd > /dev/null 2>&1

pushd libs/util > /dev/null 2>&1
cargo build --no-default-features --target-dir ../../target_util
popd > /dev/null 2>&1

cp target_ffxiv_parser/debug/libffxiv_parser.rlib ../FFXIVTools/libs/ffxiv_parser/debug/
cp -r target_ffxiv_parser/debug/deps ../FFXIVTools/libs/ffxiv_parser/debug/

cp target_sqpack_reader/debug/libsqpack_reader.rlib ../FFXIVTools/libs/sqpack_reader/debug/
cp -r target_sqpack_reader/debug/deps ../FFXIVTools/libs/sqpack_reader/debug/

cp target_util/debug/libutil.rlib ../FFXIVTools/libs/util/debug/
cp -r target_util/debug/deps ../FFXIVTools/libs/util/debug/
