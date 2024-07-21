import pytest

from imap_codec import GreetingCodec
from imap_codec.models.greeting import Greeting


def model_assert(model: Greeting, case: dict):
    assert (
        model.model_dump(mode="json") == GreetingCodec.decode(GreetingCodec.encode(case).dump())[1]
    )


@pytest.mark.parametrize(
    "case",
    [
        {"kind": "Ok", "code": "TryCreate", "text": "0"},
        {"kind": "Ok", "code": {"UidNext": 1}, "text": "p"},
        {
            "kind": "Ok",
            "code": {
                "BadCharset": {"allowed": [{"Atom": "US-ASCII"}, {"Quoted": "UTF-8"}]}
            },
            "text": "(Success)",
        },
        {
            "kind": "Ok",
            "code": {"Other": [1, 2, 3]},
            "text": "IMAP4rev1 Service Ready",
        },
    ],
)
def test_model_parse(case):
    greeting = Greeting.model_validate(case)

    model_assert(greeting, case)
