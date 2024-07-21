from typing import Any

import pytest

from imap_codec import CommandCodec, Encoded, LineFragment, LiteralFragment, LiteralMode


def test_simple_command():
    command = {"tag": "a", "body": "Noop"}
    encoded = CommandCodec.encode(command)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [LineFragment(b"a NOOP\r\n")]

def test_simple_command_dump():
    command = {"tag": "a", "body": "Noop"}
    encoded = CommandCodec.encode(command)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"a NOOP\r\n"

@pytest.fixture(scope="module")
def MULTI_FRAGMENT_COMMAND() -> dict[str, Any]:
    return {
    "tag": "A",
    "body": {
        "Login": {
            "username": {"Atom": "alice"},
            "password": {
                "String": {"Literal": {"data": list(b"\xCA\xFE"), "mode": "Sync"}}
            },
        }
    },
}

def test_multi_fragment_command(MULTI_FRAGMENT_COMMAND: dict[str, Any]):
    encoded = CommandCodec.encode(MULTI_FRAGMENT_COMMAND)
    assert isinstance(encoded, Encoded)
    fragments = list(encoded)
    assert fragments == [LineFragment(b"A LOGIN alice {2}\r\n"), LiteralFragment(b"\xca\xfe", LiteralMode.Sync), LineFragment(b"\r\n")]

def test_multi_fragment_command_dump(MULTI_FRAGMENT_COMMAND: dict[str, Any]):
    encoded = CommandCodec.encode(MULTI_FRAGMENT_COMMAND)
    assert isinstance(encoded, Encoded)
    assert encoded.dump() == b"A LOGIN alice {2}\r\n\xca\xfe\r\n"

def test_multi_fragment_command_dump_remaining(MULTI_FRAGMENT_COMMAND):
    encoded = CommandCodec.encode(MULTI_FRAGMENT_COMMAND)
    assert isinstance(encoded, Encoded)
    assert next(encoded) == LineFragment(b"A LOGIN alice {2}\r\n")
    assert encoded.dump() == b"\xca\xfe\r\n"
