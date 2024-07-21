import pytest

from imap_codec import DecodeFailed, DecodeIncomplete, IdleDoneCodec


def test_idle_done():
    buffer = b"done\r\n<remaining>"
    remaining, idle_done = IdleDoneCodec.decode(buffer)
    assert idle_done == ()
    assert remaining == b"<remaining>"

def test_idle_done_without_remaining():
    buffer = b"done\r\n"
    remaining, idle_done = IdleDoneCodec.decode(buffer)
    assert idle_done == ()
    assert remaining == b""

def test_idle_done_error_incomplete():
    buffer = b"do"
    with pytest.raises(DecodeIncomplete) as cm:
        IdleDoneCodec.decode(buffer)
    assert not cm.value.args

def test_idle_done_error_failed():
    buffer = b"done \r\n"
    with pytest.raises(DecodeFailed) as cm:
        IdleDoneCodec.decode(buffer)
    assert not cm.value.args
