from __future__ import annotations

from typing import Tuple, Union

class DecodeError(Exception):
    """
    Error during decoding.
    """

class DecodeFailed(DecodeError):
    """
    "Failed" error during decoding:
    Decoding failed.
    """

class DecodeIncomplete(DecodeError):
    """
    "Incomplete" error during decoding:
    More data is needed.
    """

class DecodeLiteralFound(DecodeError):
    """
    "LiteralFound" error during decoding:
    The decoder stopped at the beginning of literal data.
    """

class LiteralMode:
    """
    Literal mode, i.e., sync or non-sync.

    - Sync: A synchronizing literal, i.e., `{<n>}\r\n<data>`.
    - NonSync: A non-synchronizing literal according to RFC 7888, i.e., `{<n>+}\r\n<data>`.

    Warning: The non-sync literal extension must only be used when the server advertised support
             for it sending the LITERAL+ or LITERAL- capability.
    """

    Sync: LiteralMode
    NonSync: LiteralMode

class LineFragment:
    """
    Fragment of a line that is ready to be send.
    """

    def __init__(self, data: bytes):
        """
        Create a line fragment from data bytes

        :param data: Data bytes of fragment
        :raises TypeError: `data` is not byte-like
        """

    @property
    def data(self) -> bytes:
        """
        Get line fragment data bytes

        :return: Data bytes of fragment
        """

class LiteralFragment:
    """
    Fragment of a literal that may require an action before it should be send.
    """

    def __init__(self, data: bytes, mode: LiteralMode):
        """
        Create a literal fragment from data bytes and literal mode

        :param data: Data bytes of fragment
        :param mode: Literal mode
        :raises TypeError: `data` is not byte-like
        :raises TypeError: `mode` is invalid
        """

    @property
    def data(self) -> bytes:
        """
        Get literal fragment data bytes

        :return: Data bytes of fragment
        """

    @property
    def mode(self) -> LiteralMode:
        """
        Get literal fragment literal mode

        :return: Literal mode
        """

class Encoded:
    """
    An encoded message.

    This struct facilitates the implementation of IMAP client- and server implementations by
    yielding the encoding of a message through fragments. This is required, because the usage of
    literals (and some other types) may change the IMAP message flow. Thus, in many cases, it is an
    error to just "dump" a message and send it over the network.
    """

    def __iter__(self) -> Encoded: ...
    def __next__(self) -> Union[LineFragment, LiteralFragment]: ...
    def dump(self) -> bytes:
        """
        Dump the (remaining) encoded data without being guided by fragments.
        """

class GreetingCodec:
    """
    Codec for greetings.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode greeting from given bytes.

        :param bytes: Given bytes
        :raises DecodeFailed: Decoding failed.
        :raises DecodeIncomplete: More data is needed.
        :return: Tuple of remaining bytes and decoded greeting
        """

    @staticmethod
    def encode(greeting: dict) -> Encoded:
        """
        Encode greeting into fragments.

        :param bytes: Given greeting
        :return: `Encoded` type holding fragments of encoded greeting
        """

class CommandCodec:
    """
    Codec for commands.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode command from given bytes.

        :param bytes: Given bytes
        :raises DecodeFailed: Decoding failed.
        :raises DecodeIncomplete: More data is needed.
        :raises DecodeLiteralFound: The decoder stopped at the beginning of literal data.
        :return: Tuple of remaining bytes and decoded command
        """

    @staticmethod
    def encode(command: dict) -> Encoded:
        """
        Encode command into fragments.

        :param bytes: Given command
        :return: `Encoded` type holding fragments of encoded command
        """

class AuthenticateDataCodec:
    """
    Codec for authenticate data lines.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode authenticate data line from given bytes.

        :param bytes: Given bytes
        :raises DecodeFailed: Decoding failed.
        :raises DecodeIncomplete: More data is needed.
        :return: Tuple of remaining bytes and decoded authenticate data line
        """

    @staticmethod
    def encode(authenticate_data: dict) -> Encoded:
        """
        Encode authenticate data line into fragments.

        :param bytes: Given authenticate data line
        :return: `Encoded` type holding fragments of encoded authenticate data line
        """

class ResponseCodec:
    """
    Codec for responses.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, dict]:
        """
        Decode response from given bytes.

        :param bytes: Given bytes
        :raises DecodeFailed: Decoding failed.
        :raises DecodeIncomplete: More data is needed.
        :raises DecodeLiteralFound: The decoder stopped at the beginning of literal data.
        :return: Tuple of remaining bytes and decoded response
        """

    @staticmethod
    def encode(response: dict) -> Encoded:
        """
        Encode response into fragments.

        :param bytes: Given response
        :return: `Encoded` type holding fragments of encoded response
        """

class IdleDoneCodec:
    """
    Codec for idle dones.
    """

    @staticmethod
    def decode(bytes: bytes) -> Tuple[bytes, Tuple[()]]:
        """
        Decode idle done from given bytes.

        :param bytes: Given bytes
        :raises DecodeFailed: Decoding failed.
        :raises DecodeIncomplete: More data is needed.
        :raises DecodeLiteralFound: The decoder stopped at the beginning of literal data.
        :return: Tuple of remaining bytes and decoded idle done
        """

    @staticmethod
    def encode(idle_done: Tuple[()]) -> Encoded:
        """
        Encode idle done into fragments.

        :param bytes: Given idle done
        :return: `Encoded` type holding fragments of encoded idle done
        """
