import os
import shutil


def remove_cache_files():
    path = os.path.abspath(
        os.path.join(os.path.dirname(
            __file__), "..")
    )  # Go to the project root
    for root, dirs, files in os.walk(path):
        for dir in dirs:
            if (
                dir == "__pycache__"
                or dir == ".pytest_cache"
            ):
                shutil.rmtree(
                    os.path.join(root, dir))
                print(
                    f"Removed: {os.path.join(root, dir)}")
        for file in files:
            if file.endswith(".pyc") or file.endswith(
                ".pyo"
            ):
                os.remove(
                    os.path.join(root, file))
                print(
                    f"Removed: {os.path.join(root, file)}"
                )


remove_cache_files()
