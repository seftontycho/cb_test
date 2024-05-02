import os
import shutil
import subprocess

# Set paths
output_dir = "./output"
rust_build_dir = "."
cpp_source_dir = "./pkg"

lib_dir = os.path.join(output_dir, "libs")

# Build Rust project
rust_build_command = ["cargo", "build", "--release"]
subprocess.run(rust_build_command, cwd=rust_build_dir, check=True)

# Create output directory if it doesn't exist
os.makedirs(output_dir, exist_ok=True)
os.makedirs(lib_dir, exist_ok=True)

# Copy generated .so file to output directory
shutil.copy(
    os.path.join(rust_build_dir, "target/release/libcb_test.so"),
    os.path.join(lib_dir, "libcb_test.so"),
)

# Copy generated .hpp file to output directory
shutil.copy(
    os.path.join(rust_build_dir, "target/cb_test.hpp"),
    os.path.join(lib_dir, "cb_test.hpp"),
)

# Compile test.cpp (assuming g++ is available)
cpp_build_command = [
    "g++",
    "test.cpp",
    "-L./libs",
    "-lcb_test",
    "-o",
    "test",
]

subprocess.run(cpp_build_command, cwd=output_dir, check=True)
