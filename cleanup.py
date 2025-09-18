#!/usr/bin/python3
import os
import subprocess

def find_and_clean(directory):
    for root, dirs, files in os.walk(directory):
        if 'Cargo.toml' in files:
            print(f'Found Cargo.toml in: {root}')
            try:
                # Execute "cargo clean" in the directory containing Cargo.toml
                subprocess.run(['cargo', 'clean'], cwd=root, check=True)
                print(f'Successfully cleaned: {root}')
            except subprocess.CalledProcessError as e:
                print(f'Error cleaning {root}: {e}')

if __name__ == "__main__":
    base_directory = os.getcwd()  # Start from the current directory
    find_and_clean(base_directory)
