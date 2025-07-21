from binascii import unhexlify, hexlify
from colorama import Fore
from scapy.all import *
from scapy.layers.bluetooth4LE import (
    BTLE_SCAN_REQ,
    BTLE_CONNECT_REQ,
    BTLE_SCAN_RSP,
    BTLE_ADV_IND,
    BTLE_ADV,
    BTLE_CTRL,
    LL_FEATURE_REQ,
    # LL_VERSION_IND, # Import might not be strictly needed for raw byte parsing, but good practice
    BTLE_DATA,
    LL_LENGTH_REQ,
    LL_VERSION_IND, # Explicitly add if constructing via layers later
)
from scapy.layers.bluetooth import (
    ATT_Exchange_MTU_Request,
    L2CAP_Hdr,
    SM_Hdr,
    SM_Pairing_Request,
    ATT_Hdr,
)

def generate_reply_data(data):
    # received_msg = data.decode()
    raw_packet_bytes = unhexlify(data)
    # print(f"Rceived Raw Packet Bytes: {raw_packet_bytes}")
    ble_packet = BTLE_ADV(raw_packet_bytes)
    # Link type 272 is for nodic Bluetooth LE LL (see pcap linktypes)
    writer = PcapWriter("ble_test.pcap", linktype=272)
    writer.write(ble_packet)
    # wrpcap('ble_test.pcap',[ble_packet])
    ble_packet.show()
# ADV channel needs make sure the channle number > 37 and Access Address needs to be d6be898e (This access adress is fixed for adv channel)
# DATA channel needs to make sure the channel number is < 30  and Access Address needs to be 7083329a
# 0b060c0df105

# generate_reply_data("063300029aa5060a012719000019050100d6be898e00206e334c05613c020106030311180f096e696d626c652d626c6570727068020a03874358")
# ADV_IND (missing one bytes and must has crc)
# generate_reply_data("d6be898e60230000000000c002010607030d180f1805181107f0debc9a785634127856341278563400000000") # ADV_IND example
# LL_LENGTH_REQ example (Note: This example data might be incorrect for LL_LENGTH_REQ structure)
# verison_ind
# generate_reply_data("06190002c06a060a01031d00005ef500007083329a0b060c0df1050000000000")
# Scan Rsp
# generate_reply_data("06360002c06a060a01271d00005ef50000d6be898e60230000000000c002010607030d180f1805181107f0debc9a785634127856341278563400000000")

# generate_reply_data("061f0002c06a060a01031d00005ef50000d6be898e830cf37a7d65de280000000000c0000000")


# LL_VERSION_IND example (Bluetooth 5.0, Ericsson, SubVer 1)
# AA = d6be898e, Header = 03 (LLID=3, Len=6), Payload = 0d (OpCode) 09 (VersNr 5.0) 0000 (CompID Ericsson) 0001 (SubVersNr)
# generate_reply_data("d6be898e03060d0900000001")

def gen_conn_req():
    master_addr = "28:de:65:7d:7a:f3"
    # rpl_pkt = (
    #     BTLE_DATA()
    #     / L2CAP_Hdr()
    #     / SM_Hdr()
    #     / SM_Pairing_Request(
    #         iocap=0x04,
    #         oob=0,
    #         authentication=0x09,
    #         max_key_size=16,
    #         initiator_key_distribution=0x07,
    #         responder_key_distribution=0x07,
    #     )
    # )
    # rpl_pkt = BTLE_ADV(RxAdd=1) / BTLE_CONNECT_REQ(
    #         InitA=master_addr,
    #         AdvA="c0:00:00:00:00:00",
    #         AA=0x7083329A,
    #         crc_init=0x9C9A17,
    #         win_size=2,
    #         win_offset=1,
    #         interval=1,
    #         latency=0,
    #         timeout=0x64,
    #         chM=0x7CFFFFFFFF,
    #         SCA=0,
    #         hop=5,
    #     )
    rpl_pkt = (
            BTLE_DATA(SN=0, NESN=1)
            / BTLE_CTRL()
            / LL_FEATURE_REQ(feature_set="le_encryption+le_data_len_ext")
        )
    rpl_pkt.show()
        # print(hexlify(bytes(rpl_pkt)))
        # dt_flag = 1
    return hexlify(bytes(rpl_pkt))

# def pkt_raw():
#     raw_pkt = "7b070300040002f700"
#     pkt = BTLE_DATA(unhexlify(raw_pkt))
#     pkt.show()
pkt_byte = gen_conn_req()
print(pkt_byte.decode())
# print("020b07000600dc2c0009100707")
raw_pkt = "560914fb004808fb004808"
pkt = BTLE_DATA(unhexlify(raw_pkt))
pkt.show()