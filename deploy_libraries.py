#!/usr/bin/env python

import os;

libs = ["ffxiv_parser", "sqpack_reader", "util"]
profiles = ["wasm32-unknown-unknown/debug", "debug"]

packages = ''.join([' -p {0}'.format(x) for x in libs])
os.system("cargo build {0} --target-dir target_deploy".format(packages))
os.system("cargo build {0} --target wasm32-unknown-unknown --target-dir target_deploy".format(packages))

os.system("rm -rf ../FFXIVTools/libs/prebuilt/")
for profile in profiles:
    os.system("mkdir -p ../FFXIVTools/libs/prebuilt/{0}/deps".format(profile))
    for lib in libs:
        os.system("cp target_deploy/{1}/lib{0}.rlib ../FFXIVTools/libs/prebuilt/{1}/".format(lib, profile))
    os.system("cp -r target_deploy/{0}/deps/*.rmeta ../FFXIVTools/libs/prebuilt/{0}/deps/".format(profile))
    os.system("cp -r target_deploy/{0}/deps/*.rlib ../FFXIVTools/libs/prebuilt/{0}/deps/".format(profile))
    if "wasm" not in profile:
        os.system("cp -r target_deploy/{0}/deps/*.so ../FFXIVTools/libs/prebuilt/{0}/deps/".format(profile))

os.system("rm -rf target_deploy");