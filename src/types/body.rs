use crate::{
    codec::Encode,
    types::{
        core::{IString, NString, Number},
        envelope::Envelope,
    },
    List1AttributeValueOrNil, List1OrNil,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    /// Basic fields
    pub basic: BasicFields,
    /// Type-specific fields
    pub specific: SpecificFields,
}

impl Encode for Body {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        match self.specific {
            SpecificFields::Basic {
                ref type_,
                ref subtype,
            } => {
                type_.encode(writer)?;
                writer.write_all(b" ")?;
                subtype.encode(writer)?;
                writer.write_all(b" ")?;
                self.basic.encode(writer)
            }
            SpecificFields::Message {
                ref envelope,
                ref body_structure,
                number_of_lines,
            } => {
                writer.write_all(b"\"MESSAGE\" \"RFC822\" ")?;
                self.basic.encode(writer)?;
                writer.write_all(b" ")?;
                envelope.encode(writer)?;
                writer.write_all(b" ")?;
                body_structure.encode(writer)?;
                writer.write_all(b" ")?;
                write!(writer, "{}", number_of_lines)
            }
            SpecificFields::Text {
                ref subtype,
                number_of_lines,
            } => {
                writer.write_all(b"\"TEXT\" ")?;
                subtype.encode(writer)?;
                writer.write_all(b" ")?;
                self.basic.encode(writer)?;
                writer.write_all(b" ")?;
                write!(writer, "{}", number_of_lines)
            }
        }
    }
}

/// The basic fields of a non-multipart body part.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicFields {
    /// List of attribute/value pairs ([MIME-IMB].)
    pub parameter_list: Vec<(IString, IString)>,

    /// Content id ([MIME-IMB].)
    pub id: NString,

    /// Content description ([MIME-IMB].)
    pub description: NString,

    /// Content transfer encoding ([MIME-IMB].)
    pub content_transfer_encoding: IString,

    /// Size of the body in octets.
    ///
    /// Note that this size is the size in its transfer encoding
    /// and not the resulting size after any decoding.
    pub size: Number,
}

impl Encode for BasicFields {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        List1AttributeValueOrNil(&self.parameter_list).encode(writer)?;
        writer.write_all(b" ")?;
        self.id.encode(writer)?;
        writer.write_all(b" ")?;
        self.description.encode(writer)?;
        writer.write_all(b" ")?;
        self.content_transfer_encoding.encode(writer)?;
        writer.write_all(b" ")?;
        write!(writer, "{}", self.size)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecificFields {
    /// # Example (not in RFC)
    ///
    /// Single application/{voodoo, unknown, whatever, meh} is represented as "basic"
    ///
    /// ```text
    /// (
    ///     "application" "voodoo" NIL NIL NIL "7bit" 20
    ///                            ^^^ ^^^ ^^^ ^^^^^^ ^^
    ///                            |   |   |   |      | size
    ///                            |   |   |   | content transfer encoding
    ///                            |   |   | description
    ///                            |   | id
    ///                            | parameter list
    ///
    ///     NIL NIL NIL NIL
    ///     ^^^ ^^^ ^^^ ^^^
    ///     |   |   |   | location
    ///     |   |   | language
    ///     |   | disposition
    ///     | md5
    /// )
    /// ```
    Basic {
        /// A string giving the content media type name as defined in [MIME-IMB].
        type_: IString,

        /// A string giving the content subtype name as defined in [MIME-IMB].
        subtype: IString,
    },

    /// # Example (not in RFC)
    ///
    /// Single message/rfc822 is represented as "message"
    ///
    /// ```text
    /// (
    ///     "message" "rfc822" NIL NIL NIL "7bit" 123
    ///                        ^^^ ^^^ ^^^ ^^^^^^ ^^^
    ///                        |   |   |   |      | size
    ///                        |   |   |   | content transfer encoding
    ///                        |   |   | description
    ///                        |   | id
    ///                        | parameter list
    ///
    ///     # envelope
    ///     (
    ///         NIL "message.inner.subject.ljcwooqy" ((NIL NIL "extern" "company.com")) ((NIL NIL "extern" "company.com")) ((NIL NIL "extern" "company.com")) ((NIL NIL "admin" "seurity.com")) NIL NIL NIL NIL
    ///     )
    ///
    ///     # body structure
    ///     (
    ///         "text" "plain" ("charset" "us-ascii") NIL NIL "7bit" 31
    ///         2
    ///         NIL NIL NIL NIL
    ///     )
    ///
    ///     6
    ///     ^
    ///     | number of lines
    ///
    ///     NIL NIL NIL NIL
    ///     ^^^ ^^^ ^^^ ^^^
    ///     |   |   |   | location
    ///     |   |   | language
    ///     |   | disposition
    ///     | md5
    /// )
    /// ```
    ///
    /// A body type of type MESSAGE and subtype RFC822 contains, immediately after the basic fields,
    Message {
        /// the envelope structure,
        envelope: Envelope,
        /// body structure,
        body_structure: Box<BodyStructure>,
        /// and size in text lines of the encapsulated message.
        number_of_lines: Number,
    },

    /// # Example (not in RFC)
    ///
    /// Single text/plain is represented as "text"
    ///
    /// ```text
    /// (
    ///     "text" "plain" ("charset" "us-ascii") NIL NIL "7bit" 25
    ///                    ^^^^^^^^^^^^^^^^^^^^^^ ^^^ ^^^ ^^^^^^ ^^
    ///                    |                      |   |   |      | size
    ///                    |                      |   |   | content transfer encoding
    ///                    |                      |   | description
    ///                    |                      | id
    ///                    | parameter list
    ///
    ///     1
    ///     ^
    ///     | number of lines
    ///
    ///     NIL NIL NIL NIL
    ///     ^^^ ^^^ ^^^ ^^^
    ///     |   |   |   | location
    ///     |   |   | language
    ///     |   | disposition
    ///     | md5
    /// )
    /// ```
    ///
    /// A body type of type TEXT contains, immediately after the basic fields,
    Text {
        subtype: IString,
        /// the size of the body in text lines.
        number_of_lines: Number,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyStructure {
    /// For example, a simple text message of 48 lines and 2279 octets
    /// can have a body structure of:
    ///
    /// ```text
    /// ("TEXT" "PLAIN" ("CHARSET" "US-ASCII") NIL NIL "7BIT" 2279 48)
    /// ```
    Single {
        body: Body,
        /// Extension data
        ///
        /// Extension data is never returned with the BODY fetch,
        /// but can be returned with a BODYSTRUCTURE fetch.
        /// Extension data, if present, MUST be in the defined order.
        ///
        /// Any following extension data are not yet defined in this
        /// version of the protocol, and would be as described above under
        /// multipart extension data.
        extension: Option<SinglePartExtensionData>,
    },

    /// Multiple parts are indicated by parenthesis nesting.  Instead
    /// of a body type as the first element of the parenthesized list,
    /// there is a sequence of one or more nested body structures.  The
    /// second (last?!) element of the parenthesized list is the multipart
    /// subtype (mixed, digest, parallel, alternative, etc.).
    ///
    /// For example, a two part message consisting of a text and a
    /// BASE64-encoded text attachment can have a body structure of:
    ///
    /// ```text
    /// (
    ///     ("TEXT" "PLAIN" ("CHARSET" "US-ASCII") NIL NIL "7BIT" 1152 23)
    ///     ("TEXT" "PLAIN" ("CHARSET" "US-ASCII" "NAME" "cc.diff") "<960723163407.20117h@cac.washington.edu>" "Compiler diff" "BASE64" 4554 73)
    ///     "MIXED"
    /// )
    /// ```
    ///
    /// Extension data follows the multipart subtype.  Extension data
    /// is never returned with the BODY fetch, but can be returned with
    /// a BODYSTRUCTURE fetch.  Extension data, if present, MUST be in
    /// the defined order.
    ///
    /// See [ExtensionMultiPartData](struct.ExtensionMultiPartData.html).
    ///
    /// Any following extension data are not yet defined in this
    /// version of the protocol.  Such extension data can consist of
    /// zero or more NILs, strings, numbers, or potentially nested
    /// parenthesized lists of such data.  Client implementations that
    /// do a BODYSTRUCTURE fetch MUST be prepared to accept such
    /// extension data.  Server implementations MUST NOT send such
    /// extension data until it has been defined by a revision of this
    /// protocol.
    ///
    /// # Example (not in RFC)
    ///
    /// Multipart/mixed is represented as follows...
    ///
    /// ```text
    /// (
    ///     ("text" "html" ("charset" "us-ascii") NIL NIL "7bit" 28 0 NIL NIL NIL NIL)
    ///     ("text" "plain" ("charset" "us-ascii") NIL NIL "7bit" 11 0 NIL NIL NIL NIL)
    ///     "mixed" ("boundary" "xxx") NIL NIL NIL
    ///             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ///             |
    ///             | extension data
    /// )
    /// ```
    Multi {
        bodies: Vec<BodyStructure>,
        subtype: IString,
        extension_data: Option<MultiPartExtensionData>,
    },
}

impl Encode for BodyStructure {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_all(b"(")?;
        match self {
            BodyStructure::Single { body, extension } => {
                body.encode(writer)?;
                if let Some(extension) = extension {
                    writer.write_all(b" ")?;
                    extension.encode(writer)?;
                }
            }
            BodyStructure::Multi {
                bodies,
                subtype,
                extension_data,
            } => {
                for body in bodies {
                    body.encode(writer)?;
                }
                writer.write_all(b" ")?;
                subtype.encode(writer)?;

                if let Some(extension) = extension_data {
                    writer.write_all(b" ")?;
                    extension.encode(writer)?;
                }
            }
        }
        writer.write_all(b")")
    }
}

/// The extension data of a non-multipart body part are in the following order:
#[derive(Debug, Clone, PartialEq)]
pub struct SinglePartExtensionData {
    /// A string giving the body MD5 value as defined in [MD5].
    pub md5: NString,

    /// A parenthesized list with the same content and function as
    /// the body disposition for a multipart body part.
    pub disposition: Option<Option<(IString, Vec<(IString, IString)>)>>,

    /// A string or parenthesized list giving the body language
    /// value as defined in [LANGUAGE-TAGS].
    pub language: Option<Vec<IString>>,

    /// A string list giving the body content URI as defined in [LOCATION].
    pub location: Option<NString>,

    pub extension: Vec<u8>,
}

impl Encode for SinglePartExtensionData {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        self.md5.encode(writer)?;
        if let Some(ref dsp) = self.disposition {
            writer.write_all(b" ")?;

            match dsp {
                Some((s, param)) => {
                    writer.write_all(b"(")?;
                    s.encode(writer)?;
                    writer.write_all(b" ")?;
                    List1AttributeValueOrNil(&param).encode(writer)?;
                    writer.write_all(b")")?;
                }
                None => writer.write_all(b"NIL")?,
            }

            if let Some(ref lang) = self.language {
                writer.write_all(b" ")?;
                List1OrNil(lang, b" ").encode(writer)?;

                if let Some(ref loc) = self.location {
                    writer.write_all(b" ")?;
                    loc.encode(writer)?;

                    if !self.extension.is_empty() {
                        //writer.write_all(b" ")?; // TODO: Extension includes the SP for now, as it is unparsed.
                        writer.write_all(&self.extension)?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// The extension data of a multipart body part are in the following order:
///
/// # Trace (not in RFC)
///
/// ```text
/// (
///   ("text" "html"  ("charset" "us-ascii") NIL NIL "7bit" 28 0 NIL NIL NIL NIL)
///   ("text" "plain" ("charset" "us-ascii") NIL NIL "7bit" 11 0 NIL NIL NIL NIL)
///   "mixed" ("boundary" "xxx") NIL NIL NIL
///           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///           |
///           | extension multipart data
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MultiPartExtensionData {
    /// `body parameter parenthesized list`
    ///
    /// A parenthesized list of attribute/value pairs [e.g., ("foo"
    /// "bar" "baz" "rag") where "bar" is the value of "foo", and
    /// "rag" is the value of "baz"] as defined in [MIME-IMB].
    pub parameter_list: Vec<(IString, IString)>,

    /// `body disposition`
    ///
    /// A parenthesized list, consisting of a disposition type
    /// string, followed by a parenthesized list of disposition
    /// attribute/value pairs as defined in [DISPOSITION].
    pub disposition: Option<Option<(IString, Vec<(IString, IString)>)>>,

    /// `body language`
    ///
    /// A string or parenthesized list giving the body language
    /// value as defined in [LANGUAGE-TAGS].
    pub language: Option<Vec<IString>>,

    /// `body location`
    ///
    /// A string list giving the body content URI as defined in
    /// [LOCATION].
    pub location: Option<NString>,

    pub extension: Vec<u8>,
}

impl Encode for MultiPartExtensionData {
    fn encode(&self, writer: &mut impl Write) -> std::io::Result<()> {
        List1AttributeValueOrNil(&self.parameter_list).encode(writer)?;

        if let Some(ref dsp) = self.disposition {
            writer.write_all(b" ")?;

            match dsp {
                Some((s, param)) => {
                    writer.write_all(b"(")?;
                    s.encode(writer)?;
                    writer.write_all(b" ")?;
                    List1AttributeValueOrNil(&param).encode(writer)?;
                    writer.write_all(b")")?;
                }
                None => writer.write_all(b"NIL")?,
            }

            if let Some(ref lang) = self.language {
                writer.write_all(b" ")?;
                List1OrNil(lang, b" ").encode(writer)?;

                if let Some(ref loc) = self.location {
                    writer.write_all(b" ")?;
                    loc.encode(writer)?;

                    if !self.extension.is_empty() {
                        //writer.write_all(b" "); // TODO: Extension includes the SP for now, as it is unparsed.
                        writer.write_all(&self.extension)?;
                    }
                }
            }
        }

        Ok(())
    }
}
