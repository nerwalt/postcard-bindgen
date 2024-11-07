import struct

from .util import *

class Serializer:
    def __init__(self):
        self.bytes = []

    def finish(self) -> bytes:
        return bytes(self.bytes)

    def push_n(self, bytes_in):
        self.bytes.extend(bytes_in)

    def serialize_bool(self, value):
        self.serialize_number(U8_BYTES, False, 1 if value else 0)

    def serialize_number(self, n_bytes, signed, value):
        if n_bytes == U8_BYTES:
            self.bytes.append(value)
        elif n_bytes in {U16_BYTES, U32_BYTES, U64_BYTES, U128_BYTES}:
            value_b = int(value)
            buffer = varint(n_bytes, zig_zag(n_bytes, value_b) if signed else value_b)
            self.push_n(buffer)
        else:
            raise Exception("byte count not supported")

    def serialize_number_float(self, n_bytes, value):
        if n_bytes == U32_BYTES:
            b_buffer = struct.pack("<f", value)
        elif n_bytes == U64_BYTES:
            b_buffer = struct.pack("<d", value)
        else:
            raise Exception("byte count not supported")
        self.push_n(b_buffer)

    def serialize_string(self, s):
        self.push_n(varint(U32_BYTES, len(s)))
        self.push_n([ord(c) for c in s])

    def serialize_array(self, ser, array, length):
        if length is None:
            self.push_n(varint(U32_BYTES, len(array)))
        [ser(self, array[i]) for i in range(len(array) if length is None else length)]

    def serialize_map(self, ser, map_obj):
        entries = list(map_obj.items())
        self.push_n(varint(U32_BYTES, len(entries)))
        for k, v in entries:
            ser(self, k, v)
