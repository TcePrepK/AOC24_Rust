import bisect
import os
import re
import subprocess
import time

import numpy as np


# Function to parse the time value from the Rust output
def parse_time(value):
    # Match number followed by unit
    match = re.match(r"([\d.]+)([a-zµ]*)", value)
    if not match:
        raise ValueError(f"Invalid time format: {value}")
    number, unit = match.groups()
    number = float(number)
    # Convert based on unit
    if unit == "µs":
        return number * 1e-6  # Microseconds to seconds
    elif unit == "ms":
        return number * 1e-3  # Milliseconds to seconds
    elif unit == "s":
        return number  # Already in seconds
    else:
        raise ValueError(f"Unknown time unit: {unit}")


# Function to format time in a human-readable format
def format_time(seconds):
    if seconds < 1e-6:  # Less than a microsecond
        return f"{seconds * 1e9:.3f}ns"
    elif seconds < 1e-3:  # Less than a millisecond
        return f"{seconds * 1e6:.3f}µs"
    elif seconds < 1:  # Less than a second
        return f"{seconds * 1e3:.3f}ms"
    else:  # Seconds or more
        return f"{seconds:.3f}s"


# Function to compile the Rust project
def compile_rust_project(day_input):
    project_path = f"day{day_input}/src"
    try:
        subprocess.run(["cargo", "build", "--release"], cwd=project_path, check=True)
    except subprocess.CalledProcessError:
        print(f"Failed to compile project for day {day_input}")


# Function to run the Rust binary and parse its output
def run_rust_binary(day_input):
    try:
        result = subprocess.run(
            [f"target/release/day{day_input}"],
            cwd=f"day{day_input}",
            capture_output=True,
            text=True,
            check=True
        )
        output = result.stdout
        part1_time = parse_time(re.search(r"Part 1 result: .*?, took: ([\d.]+[a-zµ]*)", output).group(1))
        part2_time = parse_time(re.search(r"Part 2 result: .*?, took: ([\d.]+[a-zµ]*)", output).group(1))
        return part1_time, part2_time
    except subprocess.CalledProcessError as e:
        print(f"Error running binary:\n{e.stderr}")
        return None, None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None, None


# Function to clean data by removing outliers
def remove_outliers(data):
    q1 = np.percentile(data, 25)
    q3 = np.percentile(data, 75)
    iqr = q3 - q1
    lower_bound = q1 - 1.5 * iqr
    upper_bound = q3 + 1.5 * iqr
    return [x for x in data if lower_bound <= x <= upper_bound]


# Function to process a day's benchmarking results
def process_day(day, iterations):
    part1_times = []
    part2_times = []

    # Run the Rust binary several times
    for _ in range(iterations):
        p1_time, p2_time = run_rust_binary(day)
        if p1_time is not None and p2_time is not None:
            bisect.insort(part1_times, p1_time)
            bisect.insort(part2_times, p2_time)

    # Remove outliers (twice)
    part1_times_clean = remove_outliers(part1_times)
    part2_times_clean = remove_outliers(part2_times)

    return part1_times_clean, part2_times_clean


def main():
    day = input("Enter the day to benchmark: ")

    # Compile the day's Rust project
    print("Compiling Rust project...")
    project_path = f"day{day}/src"
    if not os.path.exists(project_path):
        print(f"Project for day {day} does not exist.")
        return
    compile_rust_project(day)

    # Process the day. With the given number of iterations (default 100)
    iterations = input("Enter the number of iterations (default 100): ")
    if iterations == "":
        iterations = 100
    elif iterations.isnumeric():
        iterations = int(iterations)
    else:
        print("Invalid input. Please enter a number.")
        return

    print(f"Running benchmark for day {day} with {iterations} iterations...")
    start_time = time.time()
    part1_clean, part2_clean = process_day(day, iterations)
    end_time = time.time()
    print(f"Benchmarking process took {end_time - start_time:.2f} seconds.")

    # Calculate the medians of the cleaned data
    part1_med = np.median(part1_clean)
    part2_med = np.median(part2_clean)
    print(f"| [Day {day}](./day{day}/src/main.rs) | {format_time(part1_med)} | {format_time(part2_med)} |")


main()
