from imap_codec import Encoded, GreetingCodec, LineFragment


def test_simple_greeting():
    greeting = {"code": None, "kind": "Ok", "text": "Hello, World!"}
    encoded = GreetingCodec.encode(greeting)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [LineFragment(b"* OK Hello, World!\r\n")]

def test_simple_greeting_dump():
    greeting = {"code": None, "kind": "Ok", "text": "Hello, World!"}
    encoded = GreetingCodec.encode(greeting)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"* OK Hello, World!\r\n"
