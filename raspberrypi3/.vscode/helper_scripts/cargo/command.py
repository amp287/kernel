import abc
from abc import ABC

class Arguement:
    def __init__(self, name_or_flag: str, arg_type: type, default: str = None, num_args = 1):
        self.name_or_flag = name_or_flag
        self.type = arg_type
        self.default = default
        self.num_args = num_args

class Command(ABC):
    @abc.abstractproperty 
    def description(self) -> str:
        pass

    @abc.abstractproperty
    def name(self) -> str:
        pass

    @abc.abstractproperty
    def args(self) -> list[Arguement]:
        """ args should be a list of Arguments """
        pass

    def run(self, args: list):
        pass

