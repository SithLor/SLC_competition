import os

def add_include_to_file(file_path, include_line):
    # Try reading the file with multiple encodings
    for encoding in ['utf-8', 'latin-1']:
        try:
            with open(file_path, 'r', encoding=encoding) as file:
                lines = file.readlines()
            break
        except UnicodeDecodeError:
            continue
    else:
        print(f"Skipping file due to encoding issues: {file_path}")
        return

    # Check if the include line already exists
    if any(include_line in line for line in lines):
        return  # Skip if the include line is already present

    # Add the include line at the top of the file
    with open(file_path, 'w', encoding=encoding) as file:
        file.write(include_line + '\n')
        file.writelines(lines)

def process_directory(directory, include_line, max_file_size_mb=5):
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.cpp') or file.endswith('.h'):
                file_path = os.path.join(root, file)
                # Skip files larger than the specified size
                if os.path.getsize(file_path) > max_file_size_mb * 1024 * 1024:
                    print(f"Skipping large file: {file_path}")
                    continue
                add_include_to_file(file_path, include_line)

if __name__ == "__main__":
    project_directory = "/workspaces/raptor"  # Replace with your project directory
    include_directive = '#include "type.h"'
    process_directory(project_directory, include_directive)