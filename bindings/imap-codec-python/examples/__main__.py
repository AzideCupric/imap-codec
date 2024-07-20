from typing import assert_never
from argparse import ArgumentParser

parser = ArgumentParser(description="IMAP codec example")
parser.add_argument("-e", "--example", choices=["parse_greeting", "parse_command", "parse_response"], required=True)
args = parser.parse_args()

# above python 3.10, we can use match-case, but we are using python >= 3.9 now
if args.example == "parse_greeting":
    from .parse_greeting import main
elif args.example == "parse_command":
    from .parse_command import main
elif args.example == "parse_response":
    from .parse_response import main
else:
    assert_never(args.example)

main()
