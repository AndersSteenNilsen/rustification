import json
import logging
import rs_nnid_tools


with open('nnids.json') as f:
    #logging.getLogger().setLevel(logging.INFO)
    #logging.info("This is python starting")
    #rs_nnid_tools.log_hello()
    nnids = json.load(f)
    if not rs_nnid_tools.nnid_tests(nnids):
        exit(1)
     