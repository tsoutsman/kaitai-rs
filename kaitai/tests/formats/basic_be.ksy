meta:
  id: basic
  endian: be

doc: |
  glTF is a format for distribution of 3D models optimized for being used in software

doc-ref: https://github.com/KhronosGroup/glTF/tree/2354846/specification/2.0#binary-gltf-layout

seq:
  - id: header
    type: u2
  - id: body
    type: s8
  - id: tail
    type: u4
