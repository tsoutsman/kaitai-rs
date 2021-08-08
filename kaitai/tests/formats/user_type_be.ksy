meta:
  id: user_type
  endian: be

seq:
  - id: header
    type: header
  - id: body
    type: body
  - id: tail
    type: header

types:
    header:
      seq:
        - id: seq_1
          type: u4
        - id: seq_2
          type: u8
      types:
        tail:
          seq:
            - id: seq_2
              type: u4
            - id: seq_7
              type: s2
    body:
      seq:
        - id: seq_1
          type: u8
        - id: seq_2
          type: u2
