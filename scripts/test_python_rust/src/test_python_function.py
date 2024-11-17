from scapy.all import *
from scapy.layers.bluetooth4LE import *
from scapy.layers.bluetooth import *
from binascii import unhexlify, hexlify
# from colorama import Fore, Back, Style, init

# send_nesn = 0
# send_sn = 0
# flag2 = False
# Implement the complete state machine for the BLE data channel
def generate_reply_data(pkt,flag2):
    # master_addr = "28:de:65:7d:7a:f3"
    raw_packet_bytes = unhexlify(pkt)
    # global send_sn
    # global send_nesn
    # global flag2
    # if dt_flag == 1:
    # print("DATA")
    ble_packet = BTLE_DATA(raw_packet_bytes)
    # print(ble_packet[BTLE_CTRL].opcode)
    if "BTLE_CTRL" in ble_packet:
        if ble_packet[BTLE_CTRL].opcode == 0x09:
            print("Got LL_FEATURE_RSP")
            raw_packet_bytes = unhexlify(pkt+'0000')
            ble_packet=BTLE_DATA(raw_packet_bytes)
    ble_packet.show()
    received_nesn = ble_packet[BTLE_DATA].NESN
    received_sn = ble_packet[BTLE_DATA].SN
    print(f"This is received nesn: {received_nesn}")
    print(f"This is received sn: {received_sn}")
    send_sn = received_nesn
    send_nesn = received_nesn
    # print(f"This is flag2: {flag2}")
    # if received_nesn == send_nesn:
    #     send_nesn = received_nesn
    #     send_sn = received_nesn
    # else:
    #     received_nesn = not received_nesn

    # received_nesn = not received_nesn
    # send_nesn = received_nesn
    # send_sn = received_nesn
    
    # rpl_pkt = BTLE_DATA(SN=send_sn,NESN=received_nesn) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')

    if "LL_FEATURE_RSP" in ble_packet:
        print("==========LL_FEATURE_RSP Received, Sent LL_LENGTH_REQ==========")
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_LENGTH_REQ(max_tx_bytes=247 + 4, max_rx_bytes=247 + 4)
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)
    elif "LL_LENGTH_RSP" in ble_packet or "LL_UNKNOWN_RSP" in ble_packet:
        print("==========LL_LENGTH_RSP Received, Sent LL_VERSION_IND==========")
        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / L2CAP_Hdr() / ATT_Hdr() / ATT_Exchange_MTU_Request(mtu=247)
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)

    elif "ATT_Exchange_MTU_Response" in ble_packet:
        print("==========ATT_Exchange_MTU_Response, Sent SM_Pairing_Request==========")
        rpl_pkt = BTLE_DATA() / L2CAP_Hdr() / SM_Hdr() / SM_Pairing_Request(
                iocap=0x04,
                oob=0,
                authentication=0x09,
                max_key_size=16,
                initiator_key_distribution=0x07,
                responder_key_distribution=0x07
        )
    elif "SM_Pairing_Response" in ble_packet:
        print("==========All Good man All Good==========")
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn, len=0, LLID=1)
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)
    elif "LL_VERSION_IND" in ble_packet:
        print("==========LL_VERSION_IND Received, Sent ATT_Exchange_MTU_Request==========")

        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_FEATURE_REQ(feature_set='le_encryption+le_data_len_ext')
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / L2CAP_Hdr() / ATT_Hdr() / ATT_Exchange_MTU_Request(mtu=247)
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        # rpl_pkt_arr[1:2] = bytearray([0x06, 0x00])
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        # print(hexlify(rpl_pkt_arr))
        rpl_pkt = bytes(rpl_pkt_arr)
    elif "BTLE_DATA"  in ble_packet:
        print("==========BTLE_DATA Received, Sent LL_FEATURE_REQ==========")
        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_FEATURE_REQ(feature_set='le_encryption+le_data_len_ext')
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)
    else:
        print("Does not recognise reply, sending empty pdu")
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn, len=0, LLID=1)
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)


    pkt_summary = hexlify(bytes(rpl_pkt))
    return hexlify(bytes(rpl_pkt)), 8, pkt_summary

def generate_reply_data_original(pkt,flag2):
    # master_addr = "28:de:65:7d:7a:f3"
    raw_packet_bytes = unhexlify(pkt)
    # global send_sn
    # global send_nesn
    # global flag2
    # if dt_flag == 1:
    print("DATA")
    ble_packet = BTLE_DATA(raw_packet_bytes)
    received_nesn = ble_packet[BTLE_DATA].NESN
    received_sn = ble_packet[BTLE_DATA].SN
    print(f"This is received nesn: {received_nesn}")
    print(f"This is received sn: {received_sn}")
    send_sn = received_nesn
    send_nesn = received_nesn
    #print(f"This is flag2: {flag2}")
    # if received_nesn == send_nesn:
    #     send_nesn = received_nesn
    #     send_sn = received_nesn
    # else:
    #     received_nesn = not received_nesn

    # received_nesn = not received_nesn
    # send_nesn = received_nesn
    # send_sn = received_nesn
    
    # rpl_pkt = BTLE_DATA(SN=send_sn,NESN=received_nesn) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
    if int(flag2) == 0:
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn, len=0, LLID=1)
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([0x00, 0x00])
        # print(hexlify(rpl_pkt_arr))
        rpl_pkt = bytes(rpl_pkt_arr)
    else:
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) /  BTLE_CTRL() / LL_VERSION_IND()
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([0x06, 0x00])
        # print(hexlify(rpl_pkt_arr))
        rpl_pkt = bytes(rpl_pkt_arr)
    

        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) /  BTLE_CTRL() / LL_FEATURE_REQ()


    pkt_summary = hexlify(bytes(rpl_pkt))
    return hexlify(bytes(rpl_pkt)), 8, pkt_summary


def generate_reply_adv(pkt):
    master_addr = "28:de:65:7d:7a:f3"
    raw_packet_bytes = unhexlify(pkt)
    print("ADV")
    ble_packet = BTLE_ADV(raw_packet_bytes)
    pkt_summary = ble_packet.summary()
    # print(Fore.RED+f"Rceived Message: {str(pkt_summary)}")
    if BTLE_ADV_IND in ble_packet:
        # send scan request
        rpl_pkt = BTLE_ADV(RxAdd=1)/BTLE_SCAN_REQ(AdvA = ble_packet[BTLE_ADV_IND].AdvA, ScanA = master_addr)
        # rpl_pkt.show()
        # print(hexlify(bytes(rpl_pkt)))
        send_pkt_summary = rpl_pkt.summary()

        return hexlify(bytes(rpl_pkt)), ble_packet[BTLE_ADV].PDU_type, send_pkt_summary
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
        # print(hexlify(bytes(rpl_pkt)))
        # dt_flag = 1
        send_pkt_summary = rpl_pkt.summary()
        return hexlify(bytes(rpl_pkt)), ble_packet[BTLE_ADV].PDU_type, send_pkt_summary

def handle_adv(data):
    received_msg = data.decode()
    print(f"Rceived Message: {str(received_msg)}")
    try:
        rpl, pkt_t, p_summary = generate_reply_adv(str(received_msg))
        return rpl
    except Exception as e:
        print("There is an error occured")
        traceback.print_exc()

# Updated to remove the extra byte of length
def handle_data(data,flag2):
    received_msg = data.decode()
    msg_lst = list(received_msg)
    msg_lst.pop(4)
    msg_lst.pop(4)
    received_msg = ''.join(msg_lst)
    print(f"Rceived Message: {str(received_msg)}")
    try:
        rpl, pkt_t, p_summary = generate_reply_data(str(received_msg),flag2)
        return rpl
    except Exception as e:
        print("There is an error occured")
        traceback.print_exc()

def generate_empty_pdu():
    rpl_pkt = BTLE_DATA(SN=0,NESN=0, LLID=1)
    send_pkt_summary = rpl_pkt.summary()
    print(f"RX <--- {str(send_pkt_summary)}")
    rpl_pkt_arr = bytearray(raw(rpl_pkt))
    rpl_pkt_arr[1:2] = bytearray([0x00, 0x00])
    # print(hexlify(rpl_pkt_arr))
    rpl_pkt = bytes(rpl_pkt_arr)
    rpl = hexlify(rpl_pkt)
    return rpl
# test1 = handle_adv(b'60230000000000c002010607030d180f1805181107f0debc9a785634127856341278563412')
# print(f"Result for test1: {test1}")
test_packet = BTLE_DATA()/BTLE_CTRL()/LL_FEATURE_RSP()
test = hexlify(bytes(test_packet))
print(test)
test2 = handle_data(b'07090009214101000000',1)
print(f"Result for test2: {test2}")
# test3 = generate_empty_pdu()
# print(f"Result for test3: {test3}")
