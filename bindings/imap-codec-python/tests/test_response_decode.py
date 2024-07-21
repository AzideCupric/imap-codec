import pytest

from imap_codec import DecodeFailed, DecodeIncomplete, DecodeLiteralFound, ResponseCodec


def test_response():
    buffer = b"* SEARCH 1\r\n<remaining>"
    remaining, response = ResponseCodec.decode(buffer)
    assert response == {"Data": {"Search": [1]}}
    assert remaining == b"<remaining>"

def test_response_without_remaining():
    buffer = b"* SEARCH 1\r\n"
    remaining, response = ResponseCodec.decode(buffer)
    assert response == {"Data": {"Search": [1]}}
    assert remaining == b""

def test_response_error_incomplete():
    buffer = b"* SEARCH 1"
    with pytest.raises(DecodeIncomplete) as cm:
        ResponseCodec.decode(buffer)
    assert not cm.value.args

def test_response_error_literal_found():
    buffer = b"* 1 FETCH (RFC822 {5}\r\n"
    with pytest.raises(DecodeLiteralFound, match="{'length': 5}"):
        ResponseCodec.decode(buffer)

def test_response_error_failed():
    buffer = b"A SEARCH\r\n"
    with pytest.raises(DecodeFailed) as cm:
        ResponseCodec.decode(buffer)
    assert not cm.value.args
