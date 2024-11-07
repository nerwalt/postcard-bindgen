from typing import Union

from .types import *
from .util import *
from .serializer import Serializer

def serialize_PROTOCOL(s, v):
    serialize_PACKET(s, v.packet)

def serialize_PACKET(s, v):
    if isinstance(v, Packet_A1):
        s.serialize_number(U32_BYTES, False, 0)
        serialize_A_1_META(s, v[0])
    else:
        raise TypeError("variant {} not exists".format(v))

def serialize_A_1_META(s, v):
    s.serialize_string(v.name)
    s.serialize_number(U16_BYTES, False, v.version)
    s.serialize_array(lambda s, v: s.serialize_number(U8_BYTES, False, v), v.payload, None)

from .type_checks import *
def serialize(value: Union[Protocol, Packet, A1Meta]) -> bytes:
    s = Serializer()

    if isinstance(value, PROTOCOL):
        assert_PROTOCOL(value)
        serialize_PROTOCOL(s, value)
    elif isinstance(value, PACKET):
        assert_PACKET(value)
        serialize_PACKET(s, value)
    elif isinstance(value, A_1_META):
        assert_A_1_META(value)
        serialize_A_1_META(s, value)
    else:
        raise TypeError("{} not serializable".format(type(value)))

    return s.finish()
