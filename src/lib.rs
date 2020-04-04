// INTERNET MESSAGE ACCESS PROTOCOL - VERSION 4rev1
//
// The Internet Message Access Protocol, Version 4rev1 (IMAP4rev1)
// allows a client to access and manipulate electronic mail messages on
// a server.  IMAP4rev1 permits manipulation of mailboxes (remote
// message folders) in a way that is functionally equivalent to local
// folders.  IMAP4rev1 also provides the capability for an offline
// client to resynchronize with the server.
//
// IMAP4rev1 includes operations for creating, deleting, and renaming
// mailboxes, checking for new messages, permanently removing messages,
// setting and clearing flags, RFC 2822 and RFC 2045 parsing, searching,
// and selective fetching of message attributes, texts, and portions
// thereof.  Messages in IMAP4rev1 are accessed by the use of numbers.
// These numbers are either message sequence numbers or unique
// identifiers.
//
// IMAP4rev1 supports a single server.  A mechanism for accessing
// configuration information to support multiple IMAP4rev1 servers is
// discussed in RFC 2244.
//
// IMAP4rev1 does not specify a means of posting mail; this function is
// handled by a mail transfer protocol such as RFC 2821.

// 2.2. Commands and Responses
//
// An IMAP4rev1 connection consists of the establishment of a
// client/server network connection, an initial greeting from the
// server, and client/server interactions.  These client/server
// interactions consist of a client command, server data, and a server
// completion result response.
//
// All interactions transmitted by client and server are in the form of
// lines, that is, strings that end with a CRLF.  The protocol receiver
// of an IMAP4rev1 client or server is either reading a line, or is
// reading a sequence of octets with a known count followed by a line.

// 2.2.1. Client Protocol Sender and Server Protocol Receiver
//
// The client command begins an operation.  Each client command is
// prefixed with an identifier (typically a short alphanumeric string,
// e.g., A0001, A0002, etc.) called a "tag".  A different tag is
// generated by the client for each command.
//
// Clients MUST follow the syntax outlined in this specification
// strictly.  It is a syntax error to send a command with missing or
// extraneous spaces or arguments.
//
// There are two cases in which a line from the client does not
// represent a complete command.  In one case, a command argument is
// quoted with an octet count (see the description of literal in String
// under Data Formats); in the other case, the command arguments require
// server feedback (see the AUTHENTICATE command).  In either case, the
// server sends a command continuation request response if it is ready
// for the octets (if appropriate) and the remainder of the command.
// This response is prefixed with the token "+".
//
//      Note: If instead, the server detected an error in the
//      command, it sends a BAD completion response with a tag
//      matching the command (as described below) to reject the
//      command and prevent the client from sending any more of the
//      command.
//
//      It is also possible for the server to send a completion
//      response for some other command (if multiple commands are
//      in progress), or untagged data.  In either case, the
//      command continuation request is still pending; the client
//      takes the appropriate action for the response, and reads
//      another response from the server.  In all cases, the client
//      MUST send a complete command (including receiving all
//      command continuation request responses and command
//      continuations for the command) before initiating a new
//      command.
//
// The protocol receiver of an IMAP4rev1 server reads a command line
// from the client, parses the command and its arguments, and transmits
// server data and a server command completion result response.

// TODO: see types/command.rs

// 2.2.2.  Server Protocol Sender and Client Protocol Receiver
//
// Data transmitted by the server to the client and status responses
// that do not indicate command completion are prefixed with the token
// "*", and are called untagged responses.
//
// Server data MAY be sent as a result of a client command, or MAY be
// sent unilaterally by the server.  There is no syntactic difference
// between server data that resulted from a specific command and server
// data that were sent unilaterally.
//
// The server completion result response indicates the success or
// failure of the operation.  It is tagged with the same tag as the
// client command which began the operation.  Thus, if more than one
// command is in progress, the tag in a server completion response
// identifies the command to which the response applies.  There are
// three possible server completion responses: OK (indicating success),
// NO (indicating failure), or BAD (indicating a protocol error such as
// unrecognized command or command syntax error).
//
// Servers SHOULD enforce the syntax outlined in this specification
// strictly.  Any client command with a protocol syntax error, including
// (but not limited to) missing or extraneous spaces or arguments,
// SHOULD be rejected, and the client given a BAD server completion
// response.
//
// The protocol receiver of an IMAP4rev1 client reads a response line
// from the server.  It then takes action on the response based upon the
// first token of the response, which can be a tag, a "*", or a "+".
//
// A client MUST be prepared to accept any server response at all times.
// This includes server data that was not requested.  Server data SHOULD
// be recorded, so that the client can reference its recorded copy
// rather than sending a command to the server to request the data.  In
// the case of certain server data, the data MUST be recorded.
//
// This topic is discussed in greater detail in the Server Responses
// section.

// TODO: see types/response.rs

// 2.3. Message Attributes

// TODO: see types/message_attributes.rs

// 2.4. Message Texts
/*
In addition to being able to fetch the full [RFC-2822] text of a
message, IMAP4rev1 permits the fetching of portions of the full
message text.  Specifically, it is possible to fetch the
[RFC-2822] message header, [RFC-2822] message body, a [MIME-IMB]
body part, or a [MIME-IMB] header.
*/

// 3. State and Flow Diagram
pub mod state;

// 4. Data Formats
// TODO: see types/core.rs

// 5. Operational Considerations
/*
   The following rules are listed here to ensure that all IMAP4rev1
   implementations interoperate properly.

5.1.    Mailbox Naming

   Mailbox names are 7-bit.  Client implementations MUST NOT attempt to
   create 8-bit mailbox names, and SHOULD interpret any 8-bit mailbox
   names returned by LIST or LSUB as UTF-8.  Server implementations
   SHOULD prohibit the creation of 8-bit mailbox names, and SHOULD NOT
   return 8-bit mailbox names in LIST or LSUB.  See section 5.1.3 for
   more information on how to represent non-ASCII mailbox names.

        Note: 8-bit mailbox names were undefined in earlier
        versions of this protocol.  Some sites used a local 8-bit
        character set to represent non-ASCII mailbox names.  Such
        usage is not interoperable, and is now formally deprecated.

   The case-insensitive mailbox name INBOX is a special name reserved to
   mean "the primary mailbox for this user on this server".  The
   interpretation of all other names is implementation-dependent.

   In particular, this specification takes no position on case
   sensitivity in non-INBOX mailbox names.  Some server implementations
   are fully case-sensitive; others preserve case of a newly-created
   name but otherwise are case-insensitive; and yet others coerce names
   to a particular case.  Client implementations MUST interact with any
   of these.  If a server implementation interprets non-INBOX mailbox
   names as case-insensitive, it MUST treat names using the
   international naming convention specially as described in section
   5.1.3.

   There are certain client considerations when creating a new mailbox
   name:

   1)    Any character which is one of the atom-specials (see the Formal
         Syntax) will require that the mailbox name be represented as a
         quoted string or literal.

   2)    CTL and other non-graphic characters are difficult to represent
         in a user interface and are best avoided.

   3)    Although the list-wildcard characters ("%" and "*") are valid
         in a mailbox name, it is difficult to use such mailbox names
         with the LIST and LSUB commands due to the conflict with
         wildcard interpretation.

   4)    Usually, a character (determined by the server implementation)
         is reserved to delimit levels of hierarchy.

   5)    Two characters, "#" and "&", have meanings by convention, and
         should be avoided except when used in that convention.

5.1.1.  Mailbox Hierarchy Naming

   If it is desired to export hierarchical mailbox names, mailbox names
   MUST be left-to-right hierarchical using a single character to
   separate levels of hierarchy.  The same hierarchy separator character
   is used for all levels of hierarchy within a single name.

5.1.2.  Mailbox Namespace Naming Convention

   By convention, the first hierarchical element of any mailbox name
   which begins with "#" identifies the "namespace" of the remainder of
   the name.  This makes it possible to disambiguate between different
   types of mailbox stores, each of which have their own namespaces.

        For example, implementations which offer access to USENET
        newsgroups MAY use the "#news" namespace to partition the
        USENET newsgroup namespace from that of other mailboxes.
        Thus, the comp.mail.misc newsgroup would have a mailbox
        name of "#news.comp.mail.misc", and the name
        "comp.mail.misc" can refer to a different object (e.g., a
        user's private mailbox).

5.1.3.  Mailbox International Naming Convention

   By convention, international mailbox names in IMAP4rev1 are specified
   using a modified version of the UTF-7 encoding described in [UTF-7].
   Modified UTF-7 may also be usable in servers that implement an
   earlier version of this protocol.

   In modified UTF-7, printable US-ASCII characters, except for "&",
   represent themselves; that is, characters with octet values 0x20-0x25
   and 0x27-0x7e.  The character "&" (0x26) is represented by the
   two-octet sequence "&-".

   All other characters (octet values 0x00-0x1f and 0x7f-0xff) are
   represented in modified BASE64, with a further modification from
   [UTF-7] that "," is used instead of "/".  Modified BASE64 MUST NOT be
   used to represent any printing US-ASCII character which can represent
   itself.

   "&" is used to shift to modified BASE64 and "-" to shift back to
   US-ASCII.  There is no implicit shift from BASE64 to US-ASCII, and
   null shifts ("-&" while in BASE64; note that "&-" while in US-ASCII
   means "&") are not permitted.  However, all names start in US-ASCII,
   and MUST end in US-ASCII; that is, a name that ends with a non-ASCII
   ISO-10646 character MUST end with a "-").

   The purpose of these modifications is to correct the following
   problems with UTF-7:

      1) UTF-7 uses the "+" character for shifting; this conflicts with
         the common use of "+" in mailbox names, in particular USENET
         newsgroup names.

      2) UTF-7's encoding is BASE64 which uses the "/" character; this
         conflicts with the use of "/" as a popular hierarchy delimiter.

      3) UTF-7 prohibits the unencoded usage of "\"; this conflicts with
         the use of "\" as a popular hierarchy delimiter.

      4) UTF-7 prohibits the unencoded usage of "~"; this conflicts with
         the use of "~" in some servers as a home directory indicator.

      5) UTF-7 permits multiple alternate forms to represent the same
         string; in particular, printable US-ASCII characters can be
         represented in encoded form.

      Although modified UTF-7 is a convention, it establishes certain
      requirements on server handling of any mailbox name with an
      embedded "&" character.  In particular, server implementations
      MUST preserve the exact form of the modified BASE64 portion of a
      modified UTF-7 name and treat that text as case-sensitive, even if
      names are otherwise case-insensitive or case-folded.

      Server implementations SHOULD verify that any mailbox name with an
      embedded "&" character, used as an argument to CREATE, is: in the
      correctly modified UTF-7 syntax, has no superfluous shifts, and
      has no encoding in modified BASE64 of any printing US-ASCII
      character which can represent itself.  However, client
      implementations MUST NOT depend upon the server doing this, and
      SHOULD NOT attempt to create a mailbox name with an embedded "&"
      character unless it complies with the modified UTF-7 syntax.

      Server implementations which export a mail store that does not
      follow the modified UTF-7 convention MUST convert to modified
      UTF-7 any mailbox name that contains either non-ASCII characters
      or the "&" character.

           For example, here is a mailbox name which mixes English,
           Chinese, and Japanese text:
           ~peter/mail/&U,BTFw-/&ZeVnLIqe-

           For example, the string "&Jjo!" is not a valid mailbox
           name because it does not contain a shift to US-ASCII
           before the "!".  The correct form is "&Jjo-!".  The
           string "&U,BTFw-&ZeVnLIqe-" is not permitted because it
           contains a superfluous shift.  The correct form is
           "&U,BTF2XlZyyKng-".

5.2.    Mailbox Size and Message Status Updates

   At any time, a server can send data that the client did not request.
   Sometimes, such behavior is REQUIRED.  For example, agents other than
   the server MAY add messages to the mailbox (e.g., new message
   delivery), change the flags of the messages in the mailbox (e.g.,
   simultaneous access to the same mailbox by multiple agents), or even
   remove messages from the mailbox.  A server MUST send mailbox size
   updates automatically if a mailbox size change is observed during the
   processing of a command.  A server SHOULD send message flag updates
   automatically, without requiring the client to request such updates
   explicitly.

   Special rules exist for server notification of a client about the
   removal of messages to prevent synchronization errors; see the
   description of the EXPUNGE response for more detail.  In particular,
   it is NOT permitted to send an EXISTS response that would reduce the
   number of messages in the mailbox; only the EXPUNGE response can do
   this.

   Regardless of what implementation decisions a client makes on
   remembering data from the server, a client implementation MUST record
   mailbox size updates.  It MUST NOT assume that any command after the
   initial mailbox selection will return the size of the mailbox.

5.3.    Response when no Command in Progress

   Server implementations are permitted to send an untagged response
   (except for EXPUNGE) while there is no command in progress.  Server
   implementations that send such responses MUST deal with flow control
   considerations.  Specifically, they MUST either (1) verify that the
   size of the data does not exceed the underlying transport's available
   window size, or (2) use non-blocking writes.

5.4.    Autologout Timer

   If a server has an inactivity autologout timer, the duration of that
   timer MUST be at least 30 minutes.  The receipt of ANY command from
   the client during that interval SHOULD suffice to reset the
   autologout timer.

5.5.    Multiple Commands in Progress

   The client MAY send another command without waiting for the
   completion result response of a command, subject to ambiguity rules
   (see below) and flow control constraints on the underlying data
   stream.  Similarly, a server MAY begin processing another command
   before processing the current command to completion, subject to
   ambiguity rules.  However, any command continuation request responses
   and command continuations MUST be negotiated before any subsequent
   command is initiated.

   The exception is if an ambiguity would result because of a command
   that would affect the results of other commands.  Clients MUST NOT
   send multiple commands without waiting if an ambiguity would result.
   If the server detects a possible ambiguity, it MUST execute commands
   to completion in the order given by the client.

   The most obvious example of ambiguity is when a command would affect
   the results of another command, e.g., a FETCH of a message's flags
   and a STORE of that same message's flags.

   A non-obvious ambiguity occurs with commands that permit an untagged
   EXPUNGE response (commands other than FETCH, STORE, and SEARCH),
   since an untagged EXPUNGE response can invalidate sequence numbers in
   a subsequent command.  This is not a problem for FETCH, STORE, or
   SEARCH commands because servers are prohibited from sending EXPUNGE
   responses while any of those commands are in progress.  Therefore, if
   the client sends any command other than FETCH, STORE, or SEARCH, it
   MUST wait for the completion result response before sending a command
   with message sequence numbers.

        Note: UID FETCH, UID STORE, and UID SEARCH are different
        commands from FETCH, STORE, and SEARCH.  If the client
        sends a UID command, it must wait for a completion result
        response before sending a command with message sequence
        numbers.

   For example, the following non-waiting command sequences are invalid:

      FETCH + NOOP + STORE
      STORE + COPY + FETCH
      COPY + COPY
      CHECK + FETCH

   The following are examples of valid non-waiting command sequences:

      FETCH + STORE + SEARCH + CHECK
      STORE + COPY + EXPUNGE

      UID SEARCH + UID SEARCH may be valid or invalid as a non-waiting
      command sequence, depending upon whether or not the second UID
      SEARCH contains message sequence numbers.
*/

pub mod codec;
pub mod parse;
pub mod types;
pub mod utils;

#[cfg(test)]
mod test {
    use crate::parse::command::command;
    use nom::AsBytes;

    #[test]
    fn test_transcript_from_rfc() {
        let transcript = [
            ('S', b"* OK IMAP4rev1 Service Ready\r\n".as_bytes()),
            ('C', b"a001 login mrc secret\r\n"),
            ('S', b"a001 OK LOGIN completed\r\n"),
            ('C', b"a002 select inbox\r\n"),
            ('S', b"* 18 EXISTS\r\n"),
            (
                'S',
                b"* FLAGS (\\Answered \\Flagged \\Deleted \\Seen \\Draft)\r\n",
            ),
            ('S', b"* 2 RECENT\r\n"),
            (
                'S',
                b"* OK [UNSEEN 17] Message 17 is the first unseen message\r\n",
            ),
            ('S', b"* OK [UIDVALIDITY 3857529045] UIDs valid\r\n"),
            ('S', b"a002 OK [READ-WRITE] SELECT completed\r\n"),
            ('C', b"a003 fetch 12 full\r\n"),
            (
                'S',
                b"* 12 FETCH (FLAGS (\\Seen) INTERNALDATE \"17-Jul-1996 02:44:25 -0700\")\r\n", // shortened...
            ),
            ('S', b"a003 OK FETCH completed\r\n"),
            ('C', b"a004 fetch 12 body[header]\r\n"),
            (
                'S',
                b"* 12 FETCH (BODY[HEADER] {3}\r\nXXX)\r\n", // shortened...
            ),
            ('S', b"a004 OK FETCH completed\r\n"),
            ('C', b"a005 store 12 +flags \\deleted\r\n"),
            ('S', b"* 12 FETCH (FLAGS (\\Seen \\Deleted))\r\n"),
            ('S', b"a005 OK +FLAGS completed\r\n"),
            ('C', b"a006 logout\r\n"),
            ('S', b"* BYE IMAP4rev1 server terminating connection\r\n"),
            ('S', b"a006 OK LOGOUT completed\r\n"),
        ];

        for (side, test) in transcript.iter() {
            match side {
                'C' => {
                    command(test).unwrap();
                }
                'S' => {
                    // FIXME: many response parsers are not implemented yet. Activate this test later.
                    // response(test).unwrap();
                }
                _ => unreachable!(),
            };
        }
    }
}