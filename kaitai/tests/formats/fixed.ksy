meta:
  id: fixed_test
  endian: be

seq:
  - id: string_contents
    contents: "test string"
  - id: byte_contents
    contents: [0xab, 0xad, 0xba, 0xbe]
  - id: variable
    type: u2
