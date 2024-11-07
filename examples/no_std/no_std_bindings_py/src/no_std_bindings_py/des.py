from typing import TypeVar, Type, cast

from .types import *
from .util import *
from .deserializer import Deserializer

def deserialize_PROTOCOL(d) -> PROTOCOL:
    return Protocol(packet = deserialize_PACKET(d))

def deserialize_PACKET(d) -> PACKET:
    variant_index = d.deserialize_number(U32_BYTES, False)
    if variant_index == 0:
        return Packet_A1(deserialize_A_1_META(d))
    else:
        raise TypeError("variant index {} not exists".format(variant_index))

def deserialize_A_1_META(d) -> A_1_META:
    return A1Meta(name = d.deserialize_string(), version = d.deserialize_number(U16_BYTES, False), payload = d.deserialize_array(lambda d: d.deserialize_number(U8_BYTES, False), None))

T = TypeVar("T", Protocol, Packet, A1Meta)
def deserialize(obj_type: Type[T], bytes: bytes) -> T:
    d = Deserializer(bytes)

    if obj_type is PROTOCOL:
        return cast(T, deserialize_PROTOCOL(d))
    elif obj_type is PACKET:
        return cast(T, deserialize_PACKET(d))
    elif obj_type is A_1_META:
        return cast(T, deserialize_A_1_META(d))
    else:
        raise TypeError("{} not deserializable".format(obj_type))
