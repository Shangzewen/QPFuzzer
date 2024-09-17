#!/usr/bin/env python3

import sys
from binascii import unhexlify
from pypatches.bluetooth4LE import BTLE_ADV, BTLE_DATA
from colorama import init, Fore

DIR_TX = 1
DIR_RX = 0

raw_pkt = unhexlify(sys.argv[1])
direction = int(sys.argv[2]) if len(sys.argv) > 2 else 0

if len(raw_pkt) > 0:

    if raw_pkt[1] > 0 and raw_pkt[1] != 0:
        ble_pkt = BTLE_ADV(raw_pkt)
    elif len(raw_pkt) == 2:
        ble_pkt = BTLE_DATA(raw_pkt)

    color = Fore.CYAN if direction == DIR_TX else Fore.GREEN
    extra = "" if direction == DIR_TX else f" {Fore.YELLOW}[INJECTED]"
    # CYAN for TX, GREEN for RX
    print(f'{color}{ble_pkt}{extra}')
    # ble_pkt.show()