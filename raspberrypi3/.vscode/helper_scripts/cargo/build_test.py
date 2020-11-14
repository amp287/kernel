import sys, getopt, os
import subprocess
import json
from command import Command, Arguement
import shutil

class BuildTest(Command):

    vscode_test_dir: str = "vscode_tests"

    @property
    def name(self) -> str:
        return "build-test"
    
    @property
    def description(self) -> str:
        return f'Builds the test executable using cargo and places a copy with the name <test_name>_test in the folder: {self.vscode_test_dir} on the workspace directory'

    @property
    def args(self) -> list[Arguement]:
        return [
                Arguement("test_name", str)
        ]

    def run(self, args):
        
        test_name = args.test_name

        completed = subprocess.run(
            [
                "cargo", "test", 
                "--test", test_name, 
                "--no-run", "--message-format=json"
            ],
            encoding="utf8",
            capture_output=True
        )
        
        print(f'Cargo build test output: \n\n {completed.stdout}')

        json_lines = completed.stdout.splitlines()

        for line in json_lines:

            if test_name in line and "\"test\":true" in line:
                line = json.loads(line)
                test_file = line["executable"]
                if test_name not in test_file:
                    raise Exception(f'Could not get test executable for test: {test_name}')

                print(f'Test File found: {test_file}')

                workspace = os.environ['workspaceFolder']
                
                test_path = os.path.join(workspace, self.vscode_test_dir)

                try:
                    os.mkdir(test_path)
                except FileExistsError:
                    pass

                test_path = os.path.join(test_path, f'{test_name}_test')
                
                shutil.copy(test_file, os.path.join(test_path, test_path))

                completed = subprocess.run(
                    [
                        "/usr/local/opt/binutils/bin/objcopy",
                        test_file,
                        "--output-target=binary",
                        f'{test_path}.img'
                    ]
                )

                completed.check_returncode()
                
                return

        
        print(f'Test file not found for test: {test_name}')
        