import pytest

from imap_codec import DecodeFailed, DecodeIncomplete, GreetingCodec


def test_greeting():
    buffer = b"* OK Hello, World!\r\n<remaining>"
    remaining, greeting = GreetingCodec.decode(buffer)
    assert greeting == {"code": None, "kind": "Ok", "text": "Hello, World!"}
    assert remaining == b"<remaining>"

def test_greeting_with_code():
    buffer = b"* OK [ALERT] Hello, World!\r\n<remaining>"
    remaining, greeting = GreetingCodec.decode(buffer)
    assert greeting == {"code": "Alert", "kind": "Ok", "text": "Hello, World!"}
    assert remaining == b"<remaining>"

def test_greeting_without_remaining():
    buffer = b"* OK Hello, World!\r\n"
    remaining, greeting = GreetingCodec.decode(buffer)
    assert greeting == {"code": None, "kind": "Ok", "text": "Hello, World!"}
    assert remaining == b""

def test_greeting_error_incomplete():
    buffer = b"* OK Hello, World!"
    with pytest.raises(DecodeIncomplete) as cm:
        GreetingCodec.decode(buffer)
    assert not cm.value.args

def test_greeting_error_failed():
    buffer = b"OK"
    with pytest.raises(DecodeFailed) as cm:
        GreetingCodec.decode(buffer)
    assert not cm.value.args
