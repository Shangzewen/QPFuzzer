from scapy.all import *
from scapy.layers.dot15d4 import Dot15d4, Dot15d4FCS
from scapy.layers.zigbee import *
from scapy.all import Dot15d4, Dot15d4Beacon, RadioTap
from binascii import hexlify, unhexlify
from colorama import Fore

import random
# from scapy.layers.zigbee import ZigbeeAssociationRequest, ZigbeeAssociationResponse, ZigbeeAppDataPayload, ZigbeeAPS, ZigbeeNWK
DIR_TX = 1
DIR_RX = 0
def parse_zigbee_packet(
    pkt_hex,
    direction,
    data_channel,
    show_pkt=False,
):

    if len(pkt_hex) == 0:
        return "[Empty Packet]"


    raw_pkt = unhexlify(pkt_hex)

    # zigbee_pkt = BTLE_DATA(raw_pkt) if data_channel else BTLE_ADV(raw_pkt)
    zigbee_pkt = Dot15d4FCS(raw_pkt)

    # CYAN for TX, Yellow for RX (Injected)
    color = Fore.CYAN if direction == DIR_TX else Fore.GREEN
    extra = "" if direction == DIR_TX else f" {Fore.YELLOW}[FUZZER INJECTED]"

    if show_pkt:
        zigbee_pkt.show()

    return f"{color}{zigbee_pkt}{extra}{Fore.RESET}"
def association_response_gen(pkt_raw):
    pkt_bytes= bytes.fromhex(pkt_raw)
    # Dot15d4: Base class for IEEE 802.15.4 frames, if want FCS can use Dot15d4FCS
    zigbee_pkt = Dot15d4FCS(pkt_bytes)
    src_addr = zigbee_pkt.src_addr      # usually short address or extended
    print(src_addr)
    dst_addr = zigbee_pkt.dest_addr     # your device
    pan_id = zigbee_pkt.dest_panid      # PAN ID
    # capability = zigbee_pkt[ZigbeeAssociationRequest].capability
    # capability = zigbee_pkt.security_capability
    assoc_rsp = (
    Dot15d4FCS(
        fcf_frametype=3,     # Data frame
        fcf_ackreq=1,
        fcf_destaddrmode=3,   # Short address
        fcf_srcaddrmode=3,   # Extended address
        fcf_panidcompress=1  # Set pan id compression to true based on the real board communication
        # dest_panid=pan_id,

    )
    / Dot15d4Cmd(
        dest_panid=pan_id,
        dest_addr=src_addr,         # send back to requester
        src_addr=0xf4ce36a420108ab2, # your extended addr
        cmd_id = 0x2
    )
    / Dot15d4CmdAssocResp(
        short_address=0xa056,         # assigned short address
        association_status=0x00             # success
    )
    )
    # assoc_rsp.show()
    print(hexlify(bytes(assoc_rsp)).decode())
    return hexlify(bytes(assoc_rsp))

def ack_gen(pkt_raw):
    pkt_bytes= bytes.fromhex(pkt_raw)
    zigbee_packet = Dot15d4FCS(pkt_bytes)
    # zigbee_pkt_cmd = Dot15d4Cmd(pkt_bytes)
    seq_number = zigbee_packet.seqnum
    cmd_id = zigbee_packet.cmd_id
    # generate ack for data req need to set teh frame pending bit to true to indicate there are more data for the end device
    if cmd_id == 4:
        ack = (
            Dot15d4FCS(
                fcf_frametype = 2,
                fcf_ackreq = 0,
                fcf_panidcompress = 0,
                fcf_destaddrmode = 0,
                fcf_srcaddrmode = 0, 
                fcf_pending = 1,
                fcf_security = 0,
                seqnum = seq_number,
            )
        )
    # generate the normal ack
    else:
        ack = (
            Dot15d4FCS(
                fcf_frametype = 2,
                fcf_ackreq = 0,
                fcf_panidcompress = 0,
                fcf_destaddrmode = 0,
                fcf_srcaddrmode = 0, 
                fcf_pending = 0,
                fcf_security = 0,
                seqnum = seq_number,
            )
        )       
    # ack.show()
    print(hexlify(bytes(ack)).decode())
    return hexlify(bytes(ack))

def beacon_rsp_gen():
    # pkt_bytes = bytes.fromhex(pkt_raw)
    beacon_rsp = (
        Dot15d4FCS(
            fcf_frametype = 0,
            fcf_ackreq = 0,
            fcf_panidcompress = 0,
            fcf_destaddrmode = 0,
            fcf_srcaddrmode = 2, 
            fcf_pending = 0,
            fcf_security = 0,
        )
        / Dot15d4Beacon(
            src_panid=0xb582,
            src_addr=0x0000,         # send back to requester
            sf_assocpermit=1,
            sf_pancoord=1,
            sf_battlifeextend=0,
            sf_sforder=15,
            sf_beaconorder=15,
            sf_finalcapslot=15,
            gts_spec_permit= 0,
            gts_spec_reserved=0,
            gts_spec_desccount=0,
        )
        / ZigBeeBeacon(
            proto_id = 0,
            device_depth=0,
            extended_pan_id = 0xf4ce36a420108ab2,
            end_device_capacity=1,
            router_capacity=1,
            stack_profile=2,
            nwkc_protocol_version=2,
            tx_offset = 16777215
        )
    )
    # beacon_rsp.show()
    print(hexlify(bytes(beacon_rsp)).decode())
    return hexlify(bytes(beacon_rsp))

def handle_adv(pkt_raw, ackd):
    # TODO: need to check ack byte?? or can just ignore lol
    # print("This is pkt_raw: ", pkt_raw.decode())
    pkt_bytes= bytes.fromhex(pkt_raw.decode())
    # print("")
    zigbee_packet = Dot15d4FCS(pkt_bytes)
    cmd_id = zigbee_packet.cmd_id
    # print(cmd_id)
    # Beacon req 
    if cmd_id == 7:
        try:
            rpl_pkt = beacon_rsp_gen()
            result_rx = parse_zigbee_packet(rpl_pkt, 0, False)
            print(f"RX <---------- {result_rx}")
            return rpl_pkt
        except Exception as e:
            print(f"There is an error occured: {e}")
            traceback.print_exc()
    # Association req
    elif cmd_id == 1:
        try:
            rpl_pkt = ack_gen(pkt_raw.decode())
            result_rx = parse_zigbee_packet(rpl_pkt, 0, False)
            print(f"RX <---------- {result_rx}")
            return rpl_pkt
        except Exception as e:
            print(f"There is an error occured: {e}")
            traceback.print_exc()
    # TODO: Data req (Need to give both ack and the Association rsp, how can i do this, maybe can have a specifical flag? Need to prepare a buffer??)
    elif cmd_id == 4 and ackd==0:
        try:
            rpl_pkt = ack_gen(pkt_raw.decode())
            result_rx = parse_zigbee_packet(rpl_pkt, 0, False)
            print(f"RX <---------- {result_rx}")
            return rpl_pkt
        except Exception as e:
            print(f"There is an error occured: {e}")
            traceback.print_exc()
    elif cmd_id == 4 and ackd==1:
        try:
            rpl_pkt = association_response_gen(pkt_raw.decode())
            result_rx = parse_zigbee_packet(rpl_pkt, 0, False)
            print(f"RX <---------- {result_rx}")
            return rpl_pkt
        except Exception as e:
            print(f"There is an error occured: {e}")
            traceback.print_exc()

