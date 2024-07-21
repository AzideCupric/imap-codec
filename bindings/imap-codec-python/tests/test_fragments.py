from imap_codec import LineFragment, LiteralFragment, LiteralMode


def test_line_fragment_data():
    data = b"a NOOP\r\n"
    fragment = LineFragment(data)
    assert fragment.data == data

def test_line_fragment_repr():
    data = b"a NOOP\r\n"
    fragment = LineFragment(data)
    assert repr(fragment) == f"LineFragment({data})"

def test_line_fragment_str():
    data = b"a NOOP\r\n"
    fragment = LineFragment(data)
    assert str(fragment) == str(data)

def test_line_fragment_eq():
    fragment1 = LineFragment(b"a NOOP\r\n")
    fragment2 = LineFragment(b"a NOOP\r\n")
    fragment3 = LineFragment(b"a LOGIN alice pass\r\n")

    assert fragment1 == fragment1
    assert fragment1 == fragment2
    assert fragment1 != fragment3

    assert fragment2 == fragment1
    assert fragment2 == fragment2
    assert fragment2 != fragment3

    assert fragment3 != fragment1
    assert fragment3 != fragment2
    assert fragment3 == fragment3


def test_literal_fragment_data():
    data = b"\x01\x02\x03\x04"
    fragment = LiteralFragment(data, LiteralMode.Sync)
    assert fragment.data == data

def test_literal_fragment_mode():
    mode = LiteralMode.Sync
    fragment = LiteralFragment(b"\x01\x02\x03\x04", mode)
    assert fragment.mode == mode

    mode = LiteralMode.NonSync
    fragment = LiteralFragment(b"\x01\x02\x03\x04", mode)
    assert fragment.mode == mode

def test_literal_fragment_repr():
    data = b"\x01\x02\x03\x04"

    mode = LiteralMode.Sync
    fragment = LiteralFragment(data, mode)
    assert repr(fragment) == f"LiteralFragment({data}, {mode})"

    mode = LiteralMode.NonSync
    fragment = LiteralFragment(data, mode)
    assert repr(fragment) == f"LiteralFragment({data}, {mode})"

def test_literal_fragment_str():
    data = b"\x01\x02\x03\x04"

    mode = LiteralMode.Sync
    fragment = LiteralFragment(data, mode)
    assert str(fragment) == f"({data}, {mode})"

    mode = LiteralMode.NonSync
    fragment = LiteralFragment(data, mode)
    assert str(fragment) == f"({data}, {mode})"

def test_literal_fragment_eq():
    fragment1 = LiteralFragment(b"data", LiteralMode.Sync)
    fragment2 = LiteralFragment(b"data", LiteralMode.Sync)
    fragment3 = LiteralFragment(b"data", LiteralMode.NonSync)
    fragment4 = LiteralFragment(b"\x01\x02\x03\x04", LiteralMode.NonSync)

    assert fragment1 == fragment1
    assert fragment1 == fragment2
    assert fragment1 != fragment3
    assert fragment1 != fragment4

    assert fragment2 == fragment1
    assert fragment2 == fragment2
    assert fragment2 != fragment3
    assert fragment2 != fragment4

    assert fragment3 != fragment1
    assert fragment3 != fragment2
    assert fragment3 == fragment3
    assert fragment3 != fragment4

    assert fragment4 != fragment1
    assert fragment4 != fragment2
    assert fragment4 != fragment3
    assert fragment4 == fragment4
