use std::convert::TryFrom;

use arbitrary::{Arbitrary, Unstructured};
use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

#[cfg(feature = "ext_enable")]
use crate::extensions::enable::CapabilityEnableOther;
#[cfg(feature = "ext_quota")]
use crate::extensions::quota::ResourceOther;
use crate::{
    command::{search::SearchKey, ListCharString, SequenceSet},
    core::{AString, Atom, AtomExt, Literal, NonEmptyVec, Quoted},
    message::{
        AuthMechanismOther, DateTime, FlagExtension, Mailbox, MailboxOther, MyNaiveDate, Tag,
    },
    response::{
        data::{Capability, QuotedChar},
        CodeOther, Text,
    },
};

macro_rules! implement_tryfrom {
    ($target:ty, $from:ty) => {
        impl<'a> Arbitrary<'a> for $target {
            fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
                match <$target>::try_from(<$from>::arbitrary(u)?) {
                    Ok(passed) => Ok(passed),
                    Err(_) => Err(arbitrary::Error::IncorrectFormat),
                }
            }
        }
    };
}

macro_rules! implement_tryfrom_t {
    ($target:ty, $from:ty) => {
        impl<'a, T> Arbitrary<'a> for $target
        where
            T: Arbitrary<'a>,
        {
            fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
                match <$target>::try_from(<$from>::arbitrary(u)?) {
                    Ok(passed) => Ok(passed),
                    Err(_) => Err(arbitrary::Error::IncorrectFormat),
                }
            }
        }
    };
}

implement_tryfrom! { Atom<'a>, &str }
implement_tryfrom! { AtomExt<'a>, &str }
implement_tryfrom! { Quoted<'a>, &str }
implement_tryfrom! { Tag<'a>, &str }
implement_tryfrom! { Text<'a>, &str }
implement_tryfrom! { ListCharString<'a>, &str }
implement_tryfrom! { QuotedChar, char }
implement_tryfrom! { Mailbox<'a>, &str }
implement_tryfrom! { Capability<'a>, Atom<'a> }
implement_tryfrom! { FlagExtension<'a>, Atom<'a> }
implement_tryfrom! { MailboxOther<'a>, AString<'a> }
#[cfg(feature = "ext_enable")]
implement_tryfrom! { CapabilityEnableOther<'a>, Atom<'a> }
#[cfg(feature = "ext_quota")]
implement_tryfrom! { ResourceOther<'a>, Atom<'a> }
implement_tryfrom! { AuthMechanismOther<'a>, Atom<'a> }
implement_tryfrom! { SequenceSet, &str }
implement_tryfrom_t! { NonEmptyVec<T>, Vec<T> }

impl<'a> Arbitrary<'a> for Literal<'a> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        match Literal::try_from(<&[u8]>::arbitrary(u)?) {
            #[cfg(not(feature = "ext_literal"))]
            Ok(passed) => Ok(passed),
            #[cfg(feature = "ext_literal")]
            Ok(mut passed) => {
                passed.sync = bool::arbitrary(u)?;
                Ok(passed)
            }
            Err(_) => Err(arbitrary::Error::IncorrectFormat),
        }
    }
}

impl<'a> Arbitrary<'a> for CodeOther<'a> {
    fn arbitrary(_: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // `CodeOther` is a fallback and should usually not be created.
        Ok(CodeOther::new_unchecked(b"IMAP-CODEC-CODE-OTHER>".as_ref()))
    }
}

impl<'a> Arbitrary<'a> for SearchKey<'a> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        fn make_search_key<'a>(u: &mut Unstructured<'a>) -> arbitrary::Result<SearchKey<'a>> {
            Ok(match u.int_in_range(0u8..=33)? {
                0 => SearchKey::SequenceSet(SequenceSet::arbitrary(u)?),
                1 => SearchKey::All,
                2 => SearchKey::Answered,
                3 => SearchKey::Bcc(AString::arbitrary(u)?),
                4 => SearchKey::Before(MyNaiveDate::arbitrary(u)?),
                5 => SearchKey::Body(AString::arbitrary(u)?),
                6 => SearchKey::Cc(AString::arbitrary(u)?),
                7 => SearchKey::Deleted,
                8 => SearchKey::Draft,
                9 => SearchKey::Flagged,
                10 => SearchKey::From(AString::arbitrary(u)?),
                11 => SearchKey::Header(AString::arbitrary(u)?, AString::arbitrary(u)?),
                12 => SearchKey::Keyword(Atom::arbitrary(u)?),
                13 => SearchKey::Larger(u32::arbitrary(u)?),
                14 => SearchKey::New,
                15 => SearchKey::Old,
                16 => SearchKey::On(MyNaiveDate::arbitrary(u)?),
                17 => SearchKey::Recent,
                18 => SearchKey::Seen,
                19 => SearchKey::SentBefore(MyNaiveDate::arbitrary(u)?),
                20 => SearchKey::SentOn(MyNaiveDate::arbitrary(u)?),
                21 => SearchKey::SentSince(MyNaiveDate::arbitrary(u)?),
                22 => SearchKey::Since(MyNaiveDate::arbitrary(u)?),
                23 => SearchKey::Smaller(u32::arbitrary(u)?),
                24 => SearchKey::Subject(AString::arbitrary(u)?),
                25 => SearchKey::Text(AString::arbitrary(u)?),
                26 => SearchKey::To(AString::arbitrary(u)?),
                27 => SearchKey::Uid(SequenceSet::arbitrary(u)?),
                28 => SearchKey::Unanswered,
                29 => SearchKey::Undeleted,
                30 => SearchKey::Undraft,
                31 => SearchKey::Unflagged,
                32 => SearchKey::Unkeyword(Atom::arbitrary(u)?),
                33 => SearchKey::Unseen,
                _ => unreachable!(),
            })
        }

        fn make_search_key_rec<'a>(
            u: &mut Unstructured<'a>,
            depth: u8,
        ) -> arbitrary::Result<SearchKey<'a>> {
            if depth == 0 {
                return make_search_key(u);
            }

            Ok(match u.int_in_range(0u8..=36)? {
                0 => SearchKey::And({
                    let keys = {
                        let len = u.arbitrary_len::<SearchKey>()?;
                        let mut tmp = Vec::with_capacity(len);

                        for _ in 0..len {
                            tmp.push(make_search_key_rec(u, depth - 1)?);
                        }

                        tmp
                    };

                    if !keys.is_empty() {
                        NonEmptyVec::try_from(keys).unwrap()
                    } else {
                        NonEmptyVec::try_from(vec![make_search_key(u)?]).unwrap()
                    }
                }),
                1 => SearchKey::SequenceSet(SequenceSet::arbitrary(u)?),
                2 => SearchKey::All,
                3 => SearchKey::Answered,
                4 => SearchKey::Bcc(AString::arbitrary(u)?),
                5 => SearchKey::Before(MyNaiveDate::arbitrary(u)?),
                6 => SearchKey::Body(AString::arbitrary(u)?),
                7 => SearchKey::Cc(AString::arbitrary(u)?),
                8 => SearchKey::Deleted,
                9 => SearchKey::Draft,
                10 => SearchKey::Flagged,
                11 => SearchKey::From(AString::arbitrary(u)?),
                12 => SearchKey::Header(AString::arbitrary(u)?, AString::arbitrary(u)?),
                13 => SearchKey::Keyword(Atom::arbitrary(u)?),
                14 => SearchKey::Larger(u32::arbitrary(u)?),
                15 => SearchKey::New,
                16 => SearchKey::Not(Box::new(make_search_key_rec(u, depth - 1)?)),
                17 => SearchKey::Old,
                18 => SearchKey::On(MyNaiveDate::arbitrary(u)?),
                19 => SearchKey::Or(
                    Box::new(make_search_key_rec(u, depth - 1)?),
                    Box::new(make_search_key_rec(u, depth - 1)?),
                ),
                20 => SearchKey::Recent,
                21 => SearchKey::Seen,
                22 => SearchKey::SentBefore(MyNaiveDate::arbitrary(u)?),
                23 => SearchKey::SentOn(MyNaiveDate::arbitrary(u)?),
                24 => SearchKey::SentSince(MyNaiveDate::arbitrary(u)?),
                25 => SearchKey::Since(MyNaiveDate::arbitrary(u)?),
                26 => SearchKey::Smaller(u32::arbitrary(u)?),
                27 => SearchKey::Subject(AString::arbitrary(u)?),
                28 => SearchKey::Text(AString::arbitrary(u)?),
                29 => SearchKey::To(AString::arbitrary(u)?),
                30 => SearchKey::Uid(SequenceSet::arbitrary(u)?),
                31 => SearchKey::Unanswered,
                32 => SearchKey::Undeleted,
                33 => SearchKey::Undraft,
                34 => SearchKey::Unflagged,
                35 => SearchKey::Unkeyword(Atom::arbitrary(u)?),
                36 => SearchKey::Unseen,
                _ => unreachable!(),
            })
        }

        make_search_key_rec(u, 7)
    }
}

impl<'a> Arbitrary<'a> for DateTime {
    fn arbitrary(_: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // FIXME(#30): make arbitrary :-)

        let local_datetime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1985, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(12, 34, 56).unwrap(),
        );

        Ok(DateTime(
            FixedOffset::east_opt(3600)
                .unwrap()
                .from_local_datetime(&local_datetime)
                .unwrap(),
        ))
    }
}

impl<'a> Arbitrary<'a> for MyNaiveDate {
    fn arbitrary(_: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        // FIXME(#30): make arbitrary!

        Ok(MyNaiveDate(NaiveDate::from_ymd_opt(2020, 2, 1).unwrap()))
    }
}