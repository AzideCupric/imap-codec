from __future__ import annotations

from collections.abc import Sequence
from typing import Literal

from pydantic import BaseModel, ConfigDict, Field


class CodecModel(BaseModel):
    model_config = ConfigDict(populate_by_name=True, extra="forbid")


class StrOther(CodecModel):
    Other: str = Field(..., alias="other")


class Atom(CodecModel):
    Atom: str = Field(..., alias="atom")


class Quoted(CodecModel):
    Quoted: str = Field(..., alias="quoted")


Charset = Atom | Quoted


CodeLiteral = Literal[
    "Alert",
    "Parse",
    "ReadOnly",
    "ReadWrite",
    "TryCreate",
    "CompressionActive",
    "OverQuota",
    "TooBig",
    "UnknownCte",
    "UidNotSticky",
]


class CodeBadCharset(CodecModel):
    class Allowed(CodecModel):
        allowed: Sequence[Charset]

    BadCharset: Allowed = Field(..., alias="bad_charset")


class CodeCapability(CodecModel):
    CapabilityLiteral = Literal[
        "Imap4Rev1",
        "LoginDisabled",
        "StartTls",
        "Idle",
        "MailboxReferrals",
        "LoginReferrals",
        "SaslTr",
        "Enable",
        "Quota",
        "QuotaSet",
        "LiteralPlus",
        "LiteralMinus",
        "Move",
        "Id",
        "Unselect",
        "Metadata",
        "MetadataServer",
        "Binary",
        "UidPlus",
    ]

    class CapabilityAuth(CodecModel):
        AuthMechanism = Literal[
            "Plain",
            "Login",
            "OAuthBearer",
            "XOAuth2",
            "ScramSha1",
            "ScramSha1Plus",
            "ScramSha256",
            "ScramSha256Plus",
            "ScramSha3_512",
            "ScramSha3_512Plus",
        ]

        Auth: AuthMechanism | StrOther = Field(..., alias="auth")

    class CapabilityCompress(CodecModel):
        class Algorithm(CodecModel):
            CompressionAlgorithm = Literal["Deflate",]

            algorithm: CompressionAlgorithm

    class CapabilityQuatoRes(CodecModel):
        Resource = Literal[
            "Storage",
            "Message",
            "Mailbox",
            "AnnotationStorage",
        ]

        QuotaRes: Resource | StrOther = Field(..., alias="quota_res")

    class CapabilitySort(CodecModel):
        SortAlgorithm = Literal["Display",]

        Sort: SortAlgorithm | StrOther | None = Field(..., alias="sort")

    class CapabilityThread(CodecModel):
        ThreadAlgorithm = Literal[
            "OrderdSubject",
            "References",
        ]

        Thread: ThreadAlgorithm | StrOther = Field(..., alias="thread")

    CapabilityItem = (
        CapabilityLiteral
        | CapabilityAuth
        | CapabilityCompress
        | CapabilityQuatoRes
        | CapabilitySort
        | CapabilityThread
        | StrOther
    )

    Capability: Sequence[CapabilityItem] = Field(..., alias="capabilities")


class CodePermanentFlags(CodecModel):
    class Flag(CodecModel):
        FlagLiteral = Literal[
            "Answered",
            "Deleted",
            "Draft",
            "Flagged",
            "Seen",
        ]

        class FlagKeyword(CodecModel):
            Keyword: str

        class FlagExtension(CodecModel):
            Extension: str

        Flag: Sequence[FlagLiteral | FlagKeyword | FlagExtension] = Field(
            ..., alias="flags"
        )

    Asterisk = Literal["Asterisk"]

    FlagPerm = Flag | Asterisk

    PermanentFlags: Sequence[FlagPerm] = Field(..., alias="permanent_flags")


class CodeUidNext(CodecModel):
    UidNext: int = Field(..., alias="uid_next")


class CodeUidValidity(CodecModel):
    UidValidity: int = Field(..., alias="uid_validity")


class CodeUnseen(CodecModel):
    Unseen: int = Field(..., alias="unseen")


class CodeReferral(CodecModel):
    Referral: str


class CodeMetadata(CodecModel):
    MetadataCodeLiteral = Literal[
        "TooMany",
        "NoPrivate",
    ]

    class MetadataCodeLongEntries(CodecModel):
        LongEntries: int = Field(..., alias="long_entries")

    class MetadataCodeMaxSize(CodecModel):
        MaxSize: int = Field(..., alias="max_size")

    MetadataCode = MetadataCodeLiteral | MetadataCodeLongEntries | MetadataCodeMaxSize

    Metadata: MetadataCode = Field(..., alias="metadata")


class CodeApendUid(CodecModel):
    class AppendUidType(CodecModel):
        uid_validity: int
        uid: int

    AppendUid: AppendUidType = Field(..., alias="append_uid")


class CodeCopyUid(CodecModel):
    class CopyUidType(CodecModel):
        class Single(CodecModel):
            Single: int

        class Range(CodecModel):
            Range: tuple[int, int]

        UidElement = Single | Range

        UidSet = Sequence[UidElement]

        uid_validity: int
        source: UidSet
        destination: UidSet


class CodeOther(CodecModel):
    Other: tuple[int, ...]


Code = (
    CodeLiteral
    | CodeBadCharset
    | CodeCapability
    | CodePermanentFlags
    | CodeUidNext
    | CodeUidValidity
    | CodeUnseen
    | CodeReferral
    | CodeMetadata
    | CodeApendUid
    | CodeCopyUid
    | CodeOther
)
