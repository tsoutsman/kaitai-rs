meta:
  id: enums
  endian: be
seq:
  - id: protocol
    type: u1
    enum: ip_protocol
  - id: protocol2
    type: u1
    enum: ip_protocol
  - id: protocol3
    type: u1
    enum: ip_protocol
enums:
  ip_protocol:
    1: icmp
    6: tcp
    17: udp
