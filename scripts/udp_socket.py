#!/usr/bin/env python3
from scapy.all import *
from scapy.layers.bluetooth4LE import *
from scapy.layers.bluetooth import *
from binascii import unhexlify, hexlify
from colorama import Fore, Back, Style, init
import socket

send_nesn = 0
send_sn = 0
flag2 = False

def generate_reply(pkt,dt_flag):
    master_addr = "28:de:65:7d:7a:f3"
    raw_packet_bytes = unhexlify(pkt)
    global send_sn
    global send_nesn
    # Can use pdu_type in the first byte to distinguish whether it belongs to BTLE_ADV or BTLE_DATA
    # header_byte = raw_packet_bytes[0]
    # pdu_type = header_byte & 0x0F
    # print(pdu_type)
    # ble_packet = ""
    global flag2
    if dt_flag == 1:
        print(Fore.YELLOW+"DATA")
        ble_packet = BTLE_DATA(raw_packet_bytes)
        received_nesn = ble_packet[BTLE_DATA].NESN
        received_sn = ble_packet[BTLE_DATA].SN
        print(f"This is received nesn: {received_nesn}")
        print(f"This is received sn: {received_sn}")

        # if received_nesn == send_nesn:
        #     send_nesn = received_nesn
        #     send_sn = received_nesn
        # else:
        #     received_nesn = not received_nesn

        # received_nesn = not received_nesn
        # send_nesn = received_nesn
        # send_sn = received_nesn
       
        # rpl_pkt = BTLE_DATA(SN=send_sn,NESN=received_nesn) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
        if flag2 is False:
            rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn, len=0, LLID=1)
            flag2 = True
        else:
            rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) /  BTLE_CTRL() / LL_VERSION_IND()
            rpl_pkt_arr = bytearray(raw(rpl_pkt))
            rpl_pkt_arr[1:2] = bytearray([0x06, 0x00])
            print(hexlify(rpl_pkt_arr))
            rpl_pkt = bytes(rpl_pkt_arr)
        

            # rpl_pkt = BTLE_DATA(SN=send_sn, NESN=send_nesn) /  BTLE_CTRL() / LL_FEATURE_REQ()


        pkt_summary = hexlify(bytes(rpl_pkt))
        # if pkt == '0100':
        #     rpl_pkt = BTLE_DATA(NESN = 1, LLID = 3, len = 6)
        #     pkt_summary = hexlify(bytes(rpl_pkt))
        #     # print(hexlify(bytes(rpl_pkt)))
        # elif pkt == '0900':
        #     # rpl_pkt = BTLE_DATA(LLID = 1)

        #     rpl_pkt = BTLE_DATA(SN = 1, NESN = 1, LLID = 3,len=6)
        #     # rpl_pkt = BTLE_DATA(MD=1,LLID=1,len=27)
        #     pkt_summary = hexlify(bytes(rpl_pkt))
        # else:
        #     print(Fore.RED+"Received new reply!!!!!!!")
        #     rpl_pkt = BTLE_DATA(LLID = 1)
        #     pkt_summary = hexlify(bytes(rpl_pkt))

            # print(hexlify(bytes(rpl_pkt)))
        # rpl_pkt = BTLE_DATA(LLID=3) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
        # return hexlify(bytes(rpl_pkt)), 8, dt_flag
        return hexlify(bytes(rpl_pkt)), 8, dt_flag, pkt_summary

    else:
        print(Fore.BLUE+"ADV")
        ble_packet = BTLE_ADV(raw_packet_bytes)
        pkt_summary = ble_packet.summary()
        # print(Fore.RED+f"Rceived Message: {str(pkt_summary)}")
        if BTLE_ADV_IND in ble_packet:
            # send scan request
            rpl_pkt = BTLE_ADV(RxAdd=1)/BTLE_SCAN_REQ(AdvA = ble_packet[BTLE_ADV_IND].AdvA, ScanA = master_addr)
            # rpl_pkt.show()
            # print(hexlify(bytes(rpl_pkt)))
            send_pkt_summary = rpl_pkt.summary()

            return hexlify(bytes(rpl_pkt)), ble_packet[BTLE_ADV].PDU_type, dt_flag, send_pkt_summary
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
            return hexlify(bytes(rpl_pkt)), ble_packet[BTLE_ADV].PDU_type, dt_flag, send_pkt_summary
            

def main():
    global flag2

    init(autoreset=True)
    # Define the server's address and port (the one where Rust is bound)
    server_address = ('127.0.0.1', 9999)  # Must match the Rust remote address
    local_address = ('127.0.0.1',7777)
    data_flag = 0
    # Create a UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(local_address)
    adv_received = 1
    while(1):
        try:
            data, server = sock.recvfrom(1024)
            # print(f'Received: {data.decode()}')
            received_msg = data.decode()
            # print(f'This is the current flag: {data_flag}')
            print(Fore.RED+f"Rceived Message: {str(received_msg)}")

            if received_msg == 'RESET':
                data_flag = 0
                adv_received = 1
                flag2 = False
            elif received_msg == 'Message received!':
                pass
            elif received_msg == 'Update Flag':
                data_flag = 1
            # connected, switch to the data channel, master need to initialte the communication by send out the data pdu
            elif received_msg == 'Connected Update Flag':
                data_flag = 1
                # rpl_pkt = BTLE_DATA(LLID=1)
                # rpl_pkt = BTLE_DATA(LLID=3) / BTLE_CTRL() / LL_VERSION_IND(version='4.2')
                rpl_pkt = BTLE_DATA(SN=0,NESN=0, LLID=1)
                rpl = hexlify(bytes(rpl_pkt))

                sock.sendto(rpl,server_address)
            else:
                rpl, pkt_t, data_flag, p_summary = generate_reply(str(received_msg),data_flag)
                # print(pkt_t)
                # print(rpl)
                if(pkt_t == 0):
                    if 1< adv_received <4:
                        print(f'Advertisement received: {adv_received} times')
                        adv_received+=1
                    elif adv_received>3:
                        adv_received = 1
                    else:
                        sock.sendto(rpl,server_address)
                        print(Fore.GREEN+f"Sent Reply: {str(p_summary)}")
                        adv_received+=1
                else:
                    sock.sendto(rpl,server_address)
                    global send_nesn
                    global send_sn
                    send_nesn = not send_nesn
                    send_sn = not send_sn
                    print(f"This is the send nesn: {send_nesn}")
                    print(f"This is the send sn: {send_sn}")
                    print(Fore.GREEN+f"Sent Reply: {str(p_summary)}")
        except Exception as e:
            print("There is an error occured")
            traceback.print_exc()

if __name__ == "__main__":
    main()
