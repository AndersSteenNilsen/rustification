from datetime import datetime
from rs_nnid_tools import mod11_test

import json
CONTROLL_FACTORS = [[3, 7, 6, 1, 8, 9, 4, 5, 2, 1], [5, 4, 3, 2, 7, 6, 5, 4, 3, 2, 1]]


#def mod11_test(numbers: [int], factors: [int]) -> bool:
#    return sum([n * f % 11 for n, f in zip(numbers, factors)]) % 11 == 0


def validate_nnid(nnid: str) -> bool:
    if not len(nnid) == 11:
        return False
    nnid_a = [int(s) for s in nnid]
    birth_date = nnid[:6]

    for i, factor_list in enumerate(CONTROLL_FACTORS):
        numbers_to_check = nnid_a[:10+i]
        if not mod11_test(numbers_to_check, factor_list):
            return False
    try:
        datetime.strptime(birth_date, '%d%m%y')
    except TypeError:
        return False

    return True



with open('testdata.json') as f:
    nnids = json.load(f)
    for nnid in nnids:
        if validate_nnid(nnid) is False:
            exit(1)
