import os
import shutil

# Define the source file and the target directory
source_file = "/workspaces/raptor/type.h"  # Replace with the path to the file you want to copy
target_root = "/workspaces/raptor/winsdk10"  # Root directory to search for subdirectories

# Walk through the directory tree
for root, dirs, files in os.walk(target_root):
    for dir_name in dirs:
        target_dir = os.path.join(root, dir_name)
        try:
            # Copy the file to the current directory
            shutil.copy(source_file, target_dir)
            print(f"Copied {source_file} to {target_dir}")
        except Exception as e:
            print(f"Failed to copy to {target_dir}: {e}")