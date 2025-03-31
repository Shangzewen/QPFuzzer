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
    LL_VERSION_IND,
    BTLE_DATA,
    LL_LENGTH_REQ,
)
from scapy.layers.bluetooth import (
    ATT_Exchange_MTU_Request,
    L2CAP_Hdr,
    SM_Hdr,
    SM_Pairing_Request,
    ATT_Hdr,
)

DIR_TX = 1
DIR_RX = 0
send_sn = 0
send_nesn = 0

def parse_ble_packet(
    pkt_hex,
    direction,
    data_channel,
    show_pkt=False,
):

    if len(pkt_hex) == 0:
        return "[Empty Packet]"


    raw_pkt = unhexlify(pkt_hex)

    if data_channel and len(raw_pkt) > 3:
        # Fix bytes for data channel
        raw_pkt = bytearray(raw_pkt)
        del raw_pkt[2]
        # raw_pkt += bytearray([0x00, 0x00])

    ble_pkt = BTLE_DATA(raw_pkt) if data_channel else BTLE_ADV(raw_pkt)

    # CYAN for TX, Yellow for RX (Injected)
    color = Fore.CYAN if direction == DIR_TX else Fore.GREEN
    extra = "" if direction == DIR_TX else f" {Fore.YELLOW}[INJECTED]"

    if show_pkt:
        ble_pkt.show()

    return f"{color}{ble_pkt}{extra}{Fore.RESET}"


# Implement the complete state machine for the BLE data channel
def generate_reply_data(pkt, flag2):
    # master_addr = "28:de:65:7d:7a:f3"
    global send_nesn, send_sn 
    raw_packet_bytes = unhexlify(pkt)
    # print(f"Rceived Raw Packet Bytes: {raw_packet_bytes}")
    ble_packet = BTLE_DATA(raw_packet_bytes)
    if "BTLE_CTRL" in ble_packet:
        if ble_packet[BTLE_CTRL].opcode == 0x09:
            print("Got LL_FEATURE_RSP")
            raw_packet_bytes = unhexlify(pkt + "0000")
            ble_packet = BTLE_DATA(raw_packet_bytes)
    elif "ATT_Hdr" in ble_packet:
        if ble_packet[ATT_Hdr].opcode == 0x03:
            print("Got ATT_Exchange_MTU_Response")
            raw_packet_bytes = unhexlify(pkt + "0000")
            ble_packet = BTLE_DATA(raw_packet_bytes)
    received_nesn = ble_packet[BTLE_DATA].NESN
    received_sn = ble_packet[BTLE_DATA].SN
    # print(f"This is received nesn: {received_nesn}")
    # print(f"This is received sn: {received_sn}")
    # Complete handle sn and nesn
    if send_sn != received_nesn:
        send_sn = (send_sn+1)%2
        # print(f"Slave has received my packet, update sn to: {send_sn}")
    # else:
        # print(f"Slave has not received my packet, keep sn to: {send_sn}")
    
    if send_nesn == received_sn:
        send_nesn = (send_nesn+1)%2
        # print(f"received new packet form slave, update nesn to: {send_nesn}")
    # else:
        # print(f"received old packet form slave, keep nesn to: {send_nesn}")

    if "LL_FEATURE_RSP" in ble_packet:
        # parse_ble_packet(raw_packet_bytes,0)
        # print("==========LL_FEATURE_RSP Received, Sent LL_LENGTH_REQ==========")
        rpl_pkt = (
            BTLE_DATA(SN=send_sn, NESN=send_nesn)
            / BTLE_CTRL()
            / LL_LENGTH_REQ(max_tx_bytes=247 + 4, max_rx_bytes=247 + 4)
        )
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)
        # rpl_pkt = generate_empty_pdu()
    elif "LL_LENGTH_RSP" in ble_packet or "LL_UNKNOWN_RSP" in ble_packet:
        # parse_ble_packet(raw_packet_bytes,0)
        
        # print("==========LL_LENGTH_RSP Received, Sent LL_VERSION_IND==========")
        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / L2CAP_Hdr() / ATT_Hdr() / ATT_Exchange_MTU_Request(mtu=247)
        rpl_pkt = (
            BTLE_DATA(SN=send_sn, NESN=send_nesn)
            / BTLE_CTRL()
            / LL_VERSION_IND(version="4.2")
        )
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)

    elif "ATT_Exchange_MTU_Response" in ble_packet:
        # parse_ble_packet(raw_packet_bytes,0)
        
        # print("==========ATT_Exchange_MTU_Response, Sent SM_Pairing_Request==========")
        rpl_pkt = (
            BTLE_DATA()
            / L2CAP_Hdr()
            / SM_Hdr()
            / SM_Pairing_Request(
                iocap=0x04,
                oob=0,
                authentication=0x09,
                max_key_size=16,
                initiator_key_distribution=0x07,
                responder_key_distribution=0x07,
            )
        )
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)

    elif "SM_Pairing_Response" in ble_packet:
        # parse_ble_packet(raw_packet_bytes,0)
        
        # print("==========All Good man All Good==========")
        rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn, len=0, LLID=1)
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        rpl_pkt = bytes(rpl_pkt_arr)
    elif "LL_VERSION_IND" in ble_packet:
        # parse_ble_packet(raw_packet_bytes,0)
        
        # print(
        #     "==========LL_VERSION_IND Received, Sent ATT_Exchange_MTU_Request=========="
        # )
        # print(
        #     "==========LL_VERSION_IND Received, Sent ATT_Exchange_MTU_Request=========="
        # )

        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_FEATURE_REQ(feature_set='le_encryption+le_data_len_ext')
        rpl_pkt = (
            BTLE_DATA(SN=send_sn, NESN=send_nesn)
            / L2CAP_Hdr()
            / ATT_Hdr()
            / ATT_Exchange_MTU_Request(mtu=247)
        )
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
        rpl_pkt_arr = bytearray(raw(rpl_pkt))
        # rpl_pkt_arr[1:2] = bytearray([0x06, 0x00])
        rpl_pkt_arr[1:2] = bytearray([bytes(rpl_pkt)[1], 0x00])
        # print(hexlify(rpl_pkt_arr))
        rpl_pkt = bytes(rpl_pkt_arr)
    elif "BTLE_DATA" in ble_packet:
        # parse_ble_packet(raw_packet_bytes,0)
        # print("==========BTLE_DATA Received, Sent LL_FEATURE_REQ==========")
        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
        rpl_pkt = (
            BTLE_DATA(SN=send_sn, NESN=send_nesn)
            / BTLE_CTRL()
            / LL_FEATURE_REQ(feature_set="le_encryption+le_data_len_ext")
        )
        # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) / BTLE_CTRL() / LL_LENGTH_REQ(max_tx_bytes=247 + 4, max_rx_bytes=247 + 4)
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


def generate_reply_adv(pkt):
    master_addr = "28:de:65:7d:7a:f3"
    raw_packet_bytes = unhexlify(pkt)
    # print("This is received Message: ", pkt)
    ble_packet = BTLE_ADV(raw_packet_bytes)
    # pkt_summary = ble_packet.summary()
    # print(Fore.RED+f"Rceived Message: {str(pkt_summary)}")
    if BTLE_ADV_IND in ble_packet:
        # parse_ble_packet(pkt,0)

        # print("==========BTLE_ADV_IND Received, Sent BTLE_SCAN_REQ==========")
        # send scan request
        rpl_pkt = BTLE_ADV(RxAdd=1) / BTLE_SCAN_REQ(
            AdvA=ble_packet[BTLE_ADV_IND].AdvA, ScanA=master_addr
        )
        # rpl_pkt.show()
        # print(hexlify(bytes(rpl_pkt)))
        send_pkt_summary = rpl_pkt.summary()

        return hexlify(bytes(rpl_pkt)), ble_packet[BTLE_ADV].PDU_type, send_pkt_summary
    elif BTLE_SCAN_RSP in ble_packet:
        # parse_ble_packet(pkt,0)

        # print("==========BTLE_SCAN_RSP Received, Sent CONNECT_REQ==========")
        # send connection req
        rpl_pkt = BTLE_ADV(RxAdd=1) / BTLE_CONNECT_REQ(
            InitA=master_addr,
            AdvA=ble_packet[BTLE_SCAN_RSP].AdvA,
            AA=0x7083329A,
            crc_init=0x9C9A17,
            win_size=2,
            win_offset=1,
            interval=16,
            latency=0,
            timeout=0x64,
            chM=0x1FFFFFFFFF,
            SCA=0,
            hop=5,
        )
        # print(hexlify(bytes(rpl_pkt)))
        # dt_flag = 1
        send_pkt_summary = rpl_pkt.summary()
        return hexlify(bytes(rpl_pkt)), ble_packet[BTLE_ADV].PDU_type, send_pkt_summary


def handle_adv(data):
    packet_summary = parse_ble_packet(data,0,False)
    print(f"RX <-- {packet_summary}")
    received_msg = data.decode()
    # print(f"Rceived Message: {str(received_msg)}")
    # print (f"Rceived raw data: {data}") 

    try:
        rpl, pkt_t, p_summary = generate_reply_adv(str(received_msg))
        result_tx = parse_ble_packet(rpl,1,False)
        print(f"TX --> {result_tx}")
        return rpl
    except Exception as e:
        print(f"There is an error occured: {e}")
        traceback.print_exc()


def handle_data(data, flag2):
    packet_summary = parse_ble_packet(data,0,True)
    print(f"RX <-- {packet_summary}")
    # print(f"This is send nesn: {send_nesn}")
    # print(f"This is send sn: {send_sn}")
    global send_nesn, send_sn 
    # print(f"Rceived data: {str(data)}")
    received_msg = data.decode()
    msg_lst = list(received_msg)
    msg_lst.pop(4)
    msg_lst.pop(4)
    received_msg = "".join(msg_lst)
    # print(f"Rceived Message: {str(received_msg)}")
    try:
        rpl, pkt_t, p_summary = generate_reply_data(str(received_msg), flag2)
        result_tx = parse_ble_packet(rpl,1,True)
        print(f"TX --> {result_tx}")
        return rpl
    except Exception as e:
        print(f"There is an error occured: {e}")
        traceback.print_exc()


def generate_empty_pdu():
    # print(f"This is send nesn: {send_nesn}")
    # print(f"This is send sn: {send_sn}")
    # global send_nesn, send_sn
    send_nesn = 0
    send_sn = 0
    # print(f"Rest the sn to: {send_sn}, and nesn to: {send_nesn}")
    rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn, LLID=1)
    rpl_pkt_arr = bytearray(raw(rpl_pkt))
    rpl_pkt_arr[1:2] = bytearray([0x00, 0x00])
    # print(hexlify(rpl_pkt_arr))
    rpl_pkt = bytes(rpl_pkt_arr)
    rpl = hexlify(rpl_pkt)
    return rpl
