import os
import re

def modify_includes_in_file(file_path):
    """Modify #include directives in a single file."""
    try:
        with open(file_path, 'r') as file:
            content = file.readlines()

        modified_content = []
        for line in content:
            modified_line = re.sub(r'#include <(.*?)>', r'#include "\1"', line)
            modified_content.append(modified_line)

        with open(file_path, 'w') as file:
            file.writelines(modified_content)

        print(f"Modified includes in {file_path}")
    except Exception as e:
        print(f"Error processing {file_path}: {e}")

def process_directory(directory):
    """Recursively process all files in a directory."""
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.h') or file.endswith('.cpp'):  # Process only relevant files
                file_path = os.path.join(root, file)
                modify_includes_in_file(file_path)

# Specify the directory to process
directory_to_process = '/workspaces/raptor/winsdk10/Include'
process_directory(directory_to_process)