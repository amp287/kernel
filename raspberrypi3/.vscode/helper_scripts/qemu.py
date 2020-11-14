#!/usr/bin/env python3
from cargo.command import Command, Arguement
import subprocess
import sys, argparse
import psutil
import time

class StartQemu(Command):
    @property
    def name(self) -> str:
        return "start"
    
    @property
    def description(self) -> str:
        return f'Start a qemu process, arguments will be passed to qemu'

    @property
    def args(self) -> list[Arguement]:
        return [
                Arguement("arguments", any, num_args="*")
        ]

    def run(self, args):
        arguments = args.arguments
        print(arguments)
        process = subprocess.Popen(['qemu-system-aarch64'] + arguments)

        print(f'qemu pid = {process.pid}')

class KillQemu(Command):
    @property
    def name(self) -> str:
        return "kill"
    
    @property
    def description(self) -> str:
        return f'kill a qemu process'

    @property
    def args(self) -> list[Arguement]:
        return []

    def run(self, args):
        
        for proc in psutil.process_iter():
            if "qemu-system-aarch64" == proc.name():
                proc.kill()
                break


commands: Command = [StartQemu(), KillQemu()]

def main(argv):

    parser = argparse.ArgumentParser(description='Utility to start or stop qemu instances')
    subparsers = parser.add_subparsers()

    for command in commands:
        sub_parse = subparsers.add_parser(name=command.name, description=command.description)
        for arg in command.args:
            sub_parse.add_argument(arg.name_or_flag, nargs=arg.num_args)

        sub_parse.set_defaults(func=command.run)

    args = parser.parse_args(argv)
    args.func(args)

if __name__ == "__main__":
    main(sys.argv[1:])
    