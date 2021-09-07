meta:
  id: repeat_test
  endian: le

seq:
  - id: pre_repeat
    type: u2
  - id: main
    type: temp
    repeat: eos
types:
  temp:
    seq:
      - id: header
        type: u2
      - id: body
        type: u4
      - id: tail
        type: u2
