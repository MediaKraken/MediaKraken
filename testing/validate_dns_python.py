import socket

hostname = "mkdevmailhogsmtp.mediakraken.media"

try:
    print(f"Standard lookup: {socket.gethostbyname(hostname)}")
except Exception as e:
    print(f"Standard lookup failed: {e}")

try:
    # This mimics what most web libraries (requests, urllib) use
    print(f"Addrinfo lookup: {socket.getaddrinfo(hostname, 80)}")
except Exception as e:
    print(f"Addrinfo failed: {e}")