from .util import *
from .types import *

def assert_PROTOCOL(v):
    assert isinstance(v, Protocol), "{} is not of type {}".format(v, Protocol)
    assert_PACKET(v.packet)

def assert_PACKET(v):
    def assert_A1(v):
        assert_A_1_META(v[0])

    if isinstance(v, Packet_A1):
        assert_A1(v)
    else:
        raise TypeError("variant {} not exists".format(v))

def assert_A_1_META(v):
    assert isinstance(v, A1Meta), "{} is not of type {}".format(v, A1Meta)
    assert isinstance(v.name, str), "{} is not a string".format(v.name)
    assert isinstance(v.version, int), "{} is not an int".format(v.version)
    assert check_bounds(U16_BYTES, False, v.version), "{} does not fit into an {}".format(v.version, U16_BYTES)
    assert isinstance(v.payload, list), "{} is not a list".format(v.payload)
    def assert_v_payload(v):
        assert isinstance(v, int), "{} is not an int".format(v)
        assert check_bounds(U8_BYTES, False, v), "{} does not fit into an {}".format(v, U8_BYTES)
    [assert_v_payload(v) for v in v.payload]
