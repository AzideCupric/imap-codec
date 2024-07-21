import pytest

from imap_codec import CommandCodec, DecodeFailed, DecodeIncomplete, DecodeLiteralFound


def test_command():
    buffer = b"a NOOP\r\n<remaining>"
    remaining, command = CommandCodec.decode(buffer)
    assert command == {"tag": "a", "body": "Noop"}
    assert remaining == b"<remaining>"


def test_command_without_remaining():
    buffer = b"a NOOP\r\n"
    remaining, command = CommandCodec.decode(buffer)
    assert command == {"tag": "a", "body": "Noop"}
    assert remaining == b""


def test_command_error_incomplete():
    buffer = b"a NOOP"
    with pytest.raises(DecodeIncomplete) as cm:
        CommandCodec.decode(buffer)
    assert not cm.value.args


def test_command_error_literal_found():
    buffer = b"a SELECT {5}\r\n"
    with pytest.raises(
        DecodeLiteralFound, match="{'tag': 'a', 'length': 5, 'mode': 'Sync'}"
    ):
        CommandCodec.decode(buffer)


def test_command_error_failed():
    buffer = b"* NOOP"
    with pytest.raises(DecodeFailed) as cm:
        CommandCodec.decode(buffer)
    assert not cm.value.args
