import socket
def main():
    # Define the server's address and port (the one where Rust is bound)
    server_address = ('127.0.0.1', 9999)  # Must match the Rust remote address
    local_address = ('127.0.0.1',7777)

    # Create a UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(local_address)
    adv_received = 1
    while(1):
        try:
            # The message to send to the Rust server
            # message = bytes(msg_buf)
            # message = b'kill socket'
            # msg = bytes(msg_buf)
            # Send the message to the Rust server
            # print(f'Sending: {msg}')
            # sock.sendto(msg, server_address)
            # Receive a response from the Rust server
            data, server = sock.recvfrom(1024)
            # print(f'Received: {data.decode()}')
            received_msg = data.decode()
            # print(received_msg)
            if(received_msg == "60230000000000c002010607030d180f1805181107f0debc9a785634127856341278563412"):
                if 1< adv_received <4:
                    print(f'Advertisement received: {adv_received} times')
                    adv_received+=1
                elif adv_received>3:
                    adv_received = 1
                else:
                    print(f'Received from Hoedur: {received_msg}')
                    adv_received+=1
                    msg = b"830cf37a7d65de2800000000000c2aba95"
                    sock.sendto(msg,server_address)
                    print(f"Sent Reply: {str(msg)}")
            elif(received_msg == "441c0000000000c015095a6570687972205065726970686572616c20444b"):
                print(f'Received from Hoedur: {received_msg}')
                msg = b"8522a942f80f51c300000000000c7083329a9c9a17020100100000006400ffffffff1f05002939"
                sock.sendto(msg,server_address)
                print(f"Sent Reply: {str(msg)}")
            else:
                print(received_msg)
        except Exception as e:
            print("There is an error occured")

if __name__ == "__main__":
    main()