meta:
  id: switch_test
  endian: be
seq:
  - id: ty
    type: u4
  - id: switch
    type:
      switch-on: ty
      cases:
        4: u4
        5: u8
