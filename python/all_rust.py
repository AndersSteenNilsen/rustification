import json
import logging
import rs_nnid_tools

with open('nnids.json') as f:
    rs_nnid_tools.log_hello()
    nnids = json.load(f)
    if not rs_nnid_tools.nnid_tests(nnids):
        exit(1)
    