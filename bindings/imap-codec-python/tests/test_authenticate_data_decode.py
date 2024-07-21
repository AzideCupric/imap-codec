import pytest

from imap_codec import AuthenticateDataCodec, DecodeFailed, DecodeIncomplete


def test_authenticate_data():
    buffer = b"VGVzdA==\r\n<remaining>"
    remaining, authenticate_data = AuthenticateDataCodec.decode(buffer)
    assert authenticate_data == {"Continue": list(b"Test")}
    assert remaining == b"<remaining>"

def test_authenticate_data_without_remaining():
    buffer = b"VGVzdA==\r\n"
    remaining, authenticate_data = AuthenticateDataCodec.decode(buffer)
    assert authenticate_data == {"Continue": list(b"Test")}
    assert remaining == b""

def test_authenticate_data_error_incomplete():
    buffer = b"VGV"
    with pytest.raises(DecodeIncomplete) as cm:
        AuthenticateDataCodec.decode(buffer)

    assert not cm.value.args

def test_authenticate_data_error_failed():
    buffer = b"VGVzdA== \r\n"
    with pytest.raises(DecodeFailed) as cm:
        AuthenticateDataCodec.decode(buffer)

    assert not cm.value.args
