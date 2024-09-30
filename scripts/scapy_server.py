from scapy.all import *
from scapy.layers.bluetooth4LE import *
from scapy.layers.bluetooth import *
from binascii import unhexlify, hexlify
# correct sequence ADV_IND -> SCAN_REQ -> SCAN_RSP -> CON_REQ -> DATA we are the sentral, emulation is for peripheral
# unhexlify convert a hexstring to raw_bytes
master_addr = "28:de:65:7d:7a:f3"
raw_packet_bytes = unhexlify('441c0000000000c015095a6570687972205065726970686572616c20444b')

# Can use pdu_type in the first byte to distinguish whether it belongs to BTLE_ADV or BTLE_DATA
header_byte = raw_packet_bytes[0]
pdu_type = header_byte & 0x0F
print(pdu_type)
# ble_packet = ""
if pdu_type > 0x04:
    print("DATA")
    ble_packet = BTLE_DATA(raw_packet_bytes)
else:
    print("ADV")
    ble_packet = BTLE_ADV(raw_packet_bytes)
    if BTLE_ADV_IND in ble_packet:
        # rpl_pkt = BTLE
        print(ble_packet[BTLE_ADV_IND].AdvA)
        # send scan request
        rpl_pkt = BTLE_ADV(RxAdd=1)/BTLE_SCAN_REQ(AdvA = ble_packet[BTLE_ADV_IND].AdvA, ScanA = master_addr)
        rpl_pkt.show()
        print(hexlify(bytes(rpl_pkt)))
    elif BTLE_SCAN_RSP in ble_packet:
        # send connection req
        rpl_pkt = BTLE_ADV(RxAdd=1)/BTLE_CONNECT_REQ(InitA = master_addr, AdvA = ble_packet[BTLE_SCAN_RSP].AdvA,
                                                     AA = 0x7083329a,
                                                     crc_init = 0x9c9a17,
                                                     win_size= 2,
                                                     win_offset = 1,
                                                     interval = 16,
                                                     latency = 0,
                                                     timeout = 0x64,
                                                     chM = 0x1fffffffff,
                                                     SCA = 0,
                                                     hop = 5
                                                     )
        print(hexlify(bytes(rpl_pkt)))
        
# ble_packet.show()


# ble_adv_packet = BTLE_ADV(RxAdd=0, TxAdd=1, ChSel=1, RFU=0, PDU_type=0, Length=35)/BTLE_ADV_IND(data=[EIR_Hdr(len=2, type=1)/EIR_Flags(flags=6), EIR_Hdr(len=7, type=3)/EIR_CompleteList16BitServiceUUIDs(
#     svc_uuids=[6157, 6159, 6149]), 
#     EIR_Hdr(len=17, type=7) / \
#         EIR_CompleteList128BitServiceUUIDs(\
#             svc_uuids=[UUID('12345678-1234-5678-1234-56789abcdef0')])],
#     AdvA='c0:00:00:00:00:00')

# print('\n\n')
# ble_adv_packet.show()
