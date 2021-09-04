meta:
  id: enums
  endian: be
seq:
  - id: protocol
    type: u2
    enum: ip_protocol
enums:
  ip_protocol:
    1: icmp
    6: tcp
    17: udp
