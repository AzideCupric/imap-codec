import pytest

from imap_codec import (
    Encoded,
    LineFragment,
    LiteralFragment,
    LiteralMode,
    ResponseCodec,
)


def test_simple_response():
    response = {"Data": {"Search": [1]}}
    encoded = ResponseCodec.encode(response)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [LineFragment(b"* SEARCH 1\r\n")]


def test_simple_response_dump():
    response = {"Data": {"Search": [1]}}
    encoded = ResponseCodec.encode(response)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"* SEARCH 1\r\n"


@pytest.fixture(scope="module")
def MULTI_FRAGMENT_RESPONSE():
    return {
        "Data": {
            "Fetch": {
                "seq": 12345,
                "items": [
                    {
                        "BodyExt": {
                            "section": None,
                            "origin": None,
                            "data": {
                                "Literal": {
                                    "data": list(b"ABCDE"),
                                    "mode": "NonSync",
                                }
                            },
                        }
                    }
                ],
            }
        },
    }


def test_multi_fragment_response(MULTI_FRAGMENT_RESPONSE):
    encoded = ResponseCodec.encode(MULTI_FRAGMENT_RESPONSE)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [
        LineFragment(b"* 12345 FETCH (BODY[] {5+}\r\n"),
        LiteralFragment(b"ABCDE", LiteralMode.NonSync),
        LineFragment(b")\r\n"),
    ]


def test_multi_fragment_response_dump(MULTI_FRAGMENT_RESPONSE):
    encoded = ResponseCodec.encode(MULTI_FRAGMENT_RESPONSE)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"* 12345 FETCH (BODY[] {5+}\r\nABCDE)\r\n"


def test_multi_fragment_response_dump_remaining(MULTI_FRAGMENT_RESPONSE):
    encoded = ResponseCodec.encode(MULTI_FRAGMENT_RESPONSE)
    assert isinstance(encoded, Encoded)
    assert next(encoded) == LineFragment(b"* 12345 FETCH (BODY[] {5+}\r\n")
    assert encoded.dump() == b"ABCDE)\r\n"
