from dataclasses import dataclass, dataclass

u8 = int
i8 = int
u16 = int
i16 = int
u32 = int
i32 = int
u64 = int
i64 = int

@dataclass
class Protocol:
    packet: Packet
class Packet:
    pass

class Packet_A1(Packet, tuple[A1Meta]):

    def __new__(cls, _0: A1Meta):
        return super(Packet_A1, cls).__new__(cls, (_0,))

    def __init__(self, _0: A1Meta):
        pass

    def __str__(self) -> str:
        return "{}{}".format("Packet_A1", super().__str__())

    def __format__(self, format_spec: str) -> str:
        return super().__format__(format_spec)

    def __repr__(self) -> str:
        return super().__repr__()
@dataclass
class A1Meta:
    name: str
    version: u16
    payload: list[u8]
