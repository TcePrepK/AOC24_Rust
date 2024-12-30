import bisect
import concurrent.futures
import os
import re
import subprocess
import time

import matplotlib.pyplot as plt
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
    if unit == "ns":
        return number * 1e-9  # Nanoseconds to seconds
    elif unit == "µs":
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


def get_time_unit_and_scale(time):
    if time < 1e-9:
        return "ps", 1e12
    elif time < 1e-6:
        return "ns", 1e9
    elif time < 1e-3:
        return "µs", 1e6
    elif time < 1:
        return "ms", 1e3
    else:
        return "s", 1


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


def main(iterations=100):
    start_time = time.time()

    # First, compile the Rust projects
    days = 0
    print("Compiling Rust projects...")
    for day in range(1, 26):
        project_path = f"day{day}/src"
        if not os.path.exists(project_path):
            break
        compile_rust_project(day)
        days += 1

    # Use ThreadPoolExecutor or ProcessPoolExecutor (code stolen from ChatGPT 4)
    results = {}
    with concurrent.futures.ProcessPoolExecutor() as executor:
        future_to_day = {
            executor.submit(process_day, day, iterations): day for day in range(1, days + 1)
        }
        for future in concurrent.futures.as_completed(future_to_day):
            day = future_to_day[future]
            try:
                part1_times, part2_times = future.result()
                if part1_times is not None and part2_times is not None:
                    results[day] = (part1_times, part2_times)
            except Exception as exc:
                print(f"Day {day} processing failed: {exc}")

    # Plot results
    fig, axes = plt.subplots(2, days, figsize=(days * 1.2, 5), sharey=False)
    for i, day in enumerate(sorted(results.keys())):
        part1_clean, part2_clean = results[day]

        # Calculate the medians of the cleaned data
        part1_med = np.median(part1_clean)
        part2_med = np.median(part2_clean)
        print(f"| [Day {day}](./day{day}/src/main.rs) | {format_time(part1_med)} | {format_time(part2_med)} |")

        # Determine the appropriate unit and scale
        time_unit, scale = get_time_unit_and_scale(max(part1_med, part2_med))
        part1_scaled = [t * scale for t in part1_clean]
        part2_scaled = [t * scale for t in part2_clean]

        # Plot Part 1
        ax1 = axes[0, i]
        ax1.boxplot(
            [part1_scaled],
            patch_artist=True,
            boxprops=dict(facecolor="lightblue", color="blue"),
            medianprops=dict(color="red", linewidth=2),
            whiskerprops=dict(color="blue"),
            capprops=dict(color="blue"),
        )
        ax1.set_title(f"Day {day} ({time_unit})", fontsize=10)
        ax1.set_xticks([])

        # Plot Part 2
        ax2 = axes[1, i]
        ax2.boxplot(
            [part2_scaled],
            patch_artist=True,
            boxprops=dict(facecolor="lightgreen", color="green"),
            medianprops=dict(color="darkred", linewidth=2),
            whiskerprops=dict(color="green"),
            capprops=dict(color="green"),
        )
        ax2.set_xticks([])

    # General title and finishing touches
    fig.suptitle("Execution Times Across Days (Part 1 and Part 2)", fontsize=18, fontweight="bold")
    plt.tight_layout()
    plt.savefig("ExecutionTimes.png")
    print("Plots saved as 'ExecutionTimes.png'.")

    # Run each day once again to get total time
    mini_timer_start = time.time()
    for day in range(1, days + 1):
        process_day(day, 1)
    mini_timer_end = time.time()
    print(f"Running each day once took {mini_timer_end - mini_timer_start:.2f} seconds.")

    end_time = time.time()
    print(f"Benchmarking process took {end_time - start_time:.2f} seconds.")


main()
