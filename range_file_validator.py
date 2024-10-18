import os
import sys

def check_folders_and_files(base_path):
    error_found = False

    for folder_name in os.listdir(base_path):
        folder_path = os.path.join(base_path, folder_name)

        if os.path.isdir(folder_path) and folder_name.isdigit():
            folder_number = int(folder_name)

            if folder_number % 1000 == 0:
                start_range = folder_number
                end_range = folder_number + 999

                for file_name in os.listdir(folder_path):
                    file_path = os.path.join(folder_path, file_name)

                    file_base_name, file_extension = os.path.splitext(file_name)

                    if file_base_name.isdigit() and file_extension == ".rs":
                        file_number = int(file_base_name)

                        if not (start_range <= file_number <= end_range):
                            print(f"Invalid file: {file_name} (Folder: {folder_name})")
                            error_found = True
                    else:
                        print(f"Non-numeric or invalid extension file: {file_name} (Folder: {folder_name})")
                        error_found = True

    if error_found:
        sys.exit(1) 

if __name__ == "__main__":
    base_path = "./"
    check_folders_and_files(base_path)
