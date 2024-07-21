from __future__ import annotations

from collections.abc import Sequence
from enum import Enum
from typing import Literal, TypeAlias

from pydantic import BaseModel, ConfigDict, Field


class CodecModel(BaseModel):
    model_config = ConfigDict(
        populate_by_name=True, extra="forbid", arbitrary_types_allowed=True
    )


class StrOther(CodecModel):
    Other: str = Field(..., alias="other")


class Atom(CodecModel):
    Atom: str = Field(..., alias="atom")


class Quoted(CodecModel):
    Quoted: str = Field(..., alias="quoted")


Charset: TypeAlias = Atom | Quoted


class CodeLiteral(str, Enum):
    Alert = "Alert"
    Parse = "Parse"
    ReadOnly = "ReadOnly"
    ReadWrite = "ReadWrite"
    TryCreate = "TryCreate"
    CompressionActive = "CompressionActive"
    OverQuota = "OverQuota"
    TooBig = "TooBig"
    UnknownCte = "UnknownCte"
    UidNotSticky = "UidNotSticky"


class CodeBadCharset(CodecModel):
    class Allowed(CodecModel):
        allowed: Sequence[Charset]

    BadCharset: Allowed = Field(..., alias="bad_charset")


class CodeCapability(CodecModel):
    class CapabilityLiteral(str, Enum):
        Imap4Rev1 = "Imap4Rev1"
        LoginDisabled = "LoginDisabled"
        StartTls = "StartTls"
        Idle = "Idle"
        MailboxReferrals = "MailboxReferrals"
        LoginReferrals = "LoginReferrals"
        SaslTr = "SaslTr"
        Enable = "Enable"
        Quota = "Quota"
        QuotaSet = "QuotaSet"
        LiteralPlus = "LiteralPlus"
        LiteralMinus = "LiteralMinus"
        Move = "Move"
        Id = "Id"
        Unselect = "Unselect"
        Metadata = "Metadata"
        MetadataServer = "MetadataServer"
        Binary = "Binary"
        UidPlus = "UidPlus"

    class CapabilityAuth(CodecModel):
        class AuthMechanism(str, Enum):
            Plain = "Plain"
            Login = "Login"
            OAuthBearer = "OAuthBearer"
            XOAuth2 = "XOAuth2"
            ScramSha1 = "ScramSha1"
            ScramSha1Plus = "ScramSha1Plus"
            ScramSha256 = "ScramSha256"
            ScramSha256Plus = "ScramSha256Plus"
            ScramSha3_512 = "ScramSha3_512"
            ScramSha3_512Plus = "ScramSha3_512Plus"

        Auth: AuthMechanism | StrOther = Field(..., alias="auth")

    class CapabilityCompress(CodecModel):
        class Algorithm(CodecModel):
            class CompressionAlgorithm(str, Enum):
                Deflate = "Deflate"

            algorithm: CompressionAlgorithm

    class CapabilityQuatoRes(CodecModel):
        class Resource(str, Enum):
            Storage = "Storage"
            Message = "Message"
            Mailbox = "Mailbox"
            AnnotationStorage = "AnnotationStorage"

        QuotaRes: Resource | StrOther = Field(..., alias="quota_res")

    class CapabilitySort(CodecModel):
        class SortAlgorithm(str, Enum):
            "Display"

        Sort: SortAlgorithm | StrOther | None = Field(..., alias="sort")

    class CapabilityThread(CodecModel):
        class ThreadAlgorithm(str, Enum):
            OrderedSubject = "OrderdSubject"
            References = "References"

        Thread: ThreadAlgorithm | StrOther = Field(..., alias="thread")

    Capability: Sequence[
        CapabilityLiteral
        | CapabilityAuth
        | CapabilityCompress
        | CapabilityQuatoRes
        | CapabilitySort
        | CapabilityThread
        | StrOther
    ] = Field(..., alias="capabilities")


class CodePermanentFlags(CodecModel):
    class Flag(CodecModel):
        class FlagLiteral(str, Enum):
            Answered = "Answered"
            Deleted = "Deleted"
            Draft = "Draft"
            Flagged = "Flagged"
            Seen = "Seen"

        class FlagKeyword(CodecModel):
            Keyword: str

        class FlagExtension(CodecModel):
            Extension: str

        Flag: Sequence[FlagLiteral | FlagKeyword | FlagExtension] = Field(
            ..., alias="flags"
        )

    PermanentFlags: Sequence[Flag | Literal["Asterisk"]] = Field(
        ..., alias="permanent_flags"
    )


class CodeUidNext(CodecModel):
    UidNext: int = Field(..., alias="uid_next")


class CodeUidValidity(CodecModel):
    UidValidity: int = Field(..., alias="uid_validity")


class CodeUnseen(CodecModel):
    Unseen: int = Field(..., alias="unseen")


class CodeReferral(CodecModel):
    Referral: str


class CodeMetadata(CodecModel):
    class MetadataCodeLiteral(str, Enum):
        TooMany = "TooMany"
        NoPrivate = "NoPrivate"

    class MetadataCodeLongEntries(CodecModel):
        LongEntries: int = Field(..., alias="long_entries")

    class MetadataCodeMaxSize(CodecModel):
        MaxSize: int = Field(..., alias="max_size")

    Metadata: MetadataCodeLiteral | MetadataCodeLongEntries | MetadataCodeMaxSize = (
        Field(..., alias="metadata")
    )


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

        uid_validity: int
        source: Sequence[Single | Range]
        destination: Sequence[Single | Range]


class CodeOther(CodecModel):
    Other: tuple[int, ...]


Code: TypeAlias = (
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
