from imap_codec import Encoded, IdleDoneCodec, LineFragment


def test_idle_done():
    idle_done = ()
    encoded = IdleDoneCodec.encode(idle_done)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [LineFragment(b"DONE\r\n")]

def test_idle_done_dump():
    idle_done = ()
    encoded = IdleDoneCodec.encode(idle_done)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"DONE\r\n"
