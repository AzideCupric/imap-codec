from imap_codec import AuthenticateDataCodec, Encoded, LineFragment


def test_authenticate_data():
    authenticate_data = {"Continue": list(b"Test")}
    encoded = AuthenticateDataCodec.encode(authenticate_data)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [LineFragment(b"VGVzdA==\r\n")]

def test_authenticate_data_dump():
    authenticate_data = {"Continue": list(b"Test")}
    encoded = AuthenticateDataCodec.encode(authenticate_data)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"VGVzdA==\r\n"
