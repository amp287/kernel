#!/usr/bin/env python3

import sys, argparse
from build_test import BuildTest
from command import Command

commands: Command = [BuildTest()]

def main(argv):

    parser = argparse.ArgumentParser(description='Wrapper around cargo to simplify running tasks for Visual Studio Code')
    subparsers = parser.add_subparsers()

    for command in commands:
        sub_parse = subparsers.add_parser(name=command.name, description=command.description)
        for arg in command.args:
            sub_parse.add_argument(arg.name_or_flag, type=arg.type, default=arg.default)

        sub_parse.set_defaults(func=command.run)

    args = parser.parse_args(argv)
    args.func(args)

if __name__ == "__main__":
    main(sys.argv[1:])
    