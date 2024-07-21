from __future__ import annotations

from enum import Enum

from pydantic import BaseModel

from .common import Code


class Greeting(BaseModel):
    kind: GreetingKind
    code: Code | None
    text: str | None

class GreetingKind(str, Enum):
    Ok = "Ok"
    PreAuth = "PreAuth"
    Bye = "Bye"
