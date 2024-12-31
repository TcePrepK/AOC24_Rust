import bisect
import concurrent.futures
import re
import subprocess
import time

import matplotlib.pyplot as plt
import numpy as np
from matplotlib import ticker


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
        subprocess.run(
            ["cargo", "build", "--color=always", "--release"],
            cwd=project_path, check=True)
    except subprocess.CalledProcessError:
        print(f"Failed to compile project for day {day_input}")


# Function to run the Rust binary and parse its output
def run_rust_binary(day_input):
    try:
        result = subprocess.run(
            [f"target/release/day{day_input}"],
            cwd="./",
            capture_output=True,
            text=True,
            check=True
        )

        output = result.stdout
        search_1 = re.search(r"Part-1 \( (.*?) \) - ([\d.]+[a-zµ]*)", output)
        search_2 = re.search(r"Part-2 \( (.*?) \) - ([\d.]+[a-zµ]*)", output)

        part1_result = search_1.group(1)
        part2_result = search_2.group(1)
        part1_time = parse_time(search_1.group(2))
        part2_time = parse_time(search_2.group(2))

        return part1_time, part2_time, part1_result, part2_result
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

    p1_result = ""
    p2_result = ""

    # Run the Rust binary several times
    for _ in range(iterations):
        p1_time, p2_time, p1_result, p2_result = run_rust_binary(day)
        if p1_time is not None and p2_time is not None:
            bisect.insort(part1_times, p1_time)
            bisect.insort(part2_times, p2_time)

    # Remove outliers (twice)
    part1_times_clean = remove_outliers(part1_times)
    part2_times_clean = remove_outliers(part2_times)

    return part1_times_clean, part2_times_clean, p1_result, p2_result


def main(iterations=100):
    # First, compile the Rust projects
    days = 25
    print("Compiling Rust projects...")
    for day in range(1, days + 1):
        compile_rust_project(day)

    start_time = time.time()

    # Use ThreadPoolExecutor or ProcessPoolExecutor (code stolen from ChatGPT 4.o)
    print("Processing days...")
    results = {}
    with concurrent.futures.ProcessPoolExecutor() as executor:
        future_to_day = {
            executor.submit(process_day, day, iterations): day for day in range(1, days + 1)
        }
        for future in concurrent.futures.as_completed(future_to_day):
            day = future_to_day[future]
            try:
                part1_times, part2_times, part1_result, part2_result = future.result()
                if part1_times is not None and part2_times is not None:
                    results[day] = (part1_times, part2_times, part1_result, part2_result)
            except Exception as exc:
                print(f"Day {day} processing failed: {exc}")

    # Plot results
    part1_medians = []
    part2_medians = []

    fig, axes = plt.subplots(2, days, figsize=(days * 1.2, 5), sharey=False)
    for i, day in enumerate(sorted(results.keys())):
        part1_clean, part2_clean, _, _ = results[day]

        # Calculate the medians of the cleaned data
        part1_med = np.median(part1_clean)
        part2_med = np.median(part2_clean)
        part1_medians.append(part1_med)
        part2_medians.append(part2_med)
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

    end_time = time.time()
    print(f"Benchmarking process took {end_time - start_time:.2f} seconds.")

    # Get the total median value to use for total time
    total_median = sum(part1_medians) + sum(part2_medians)
    print(f"Running each day once took {total_median:.2f} seconds.")

    print()
    # Open (or create) a file in the writing mode ('w')
    with open("results.txt", "w") as file:
        for day in range(1, days + 1):
            file.write(f"Day {day}:\n")
            file.write(f" |> {results[day][2]}\n")
            file.write(f" |> {results[day][3]}\n")
    print("Results saved as 'results.txt'.")

    # General title and finishing touches
    fig.suptitle("Runtimes Across Days (Part 1 and Part 2)", fontsize=18, fontweight="bold")
    plt.tight_layout()
    plt.savefig("executionTimesBox.png")
    plt.close()
    print("Plots saved as 'executionTimesBox.png'.")

    # Generate the labels and bars of the graph
    labels = [f"Day {i}" for i in range(1, days + 1)]
    values_1 = [item * 1000 for item in part1_medians]
    values_2 = [item * 1000 for item in part2_medians]

    x = np.arange(len(labels))

    # Setup the whole graph
    width = 0.45
    fig, ax = plt.subplots(figsize=(days * 1.5, 7))
    rects1 = ax.bar(x - width / 2, values_1, width, label='Part 1')
    rects2 = ax.bar(x + width / 2, values_2, width, label='Part 2')

    # Add some text for labels, title and custom x-axis tick labels, etc.
    ax.set_xticks(x)
    ax.set_xticklabels(labels)
    ax.set_xlim(-0.5, len(labels) - 0.5)
    ax.yaxis.set_major_locator(ticker.MultipleLocator(10))
    plt.ylabel("Runtime (ms)", fontsize=14, color='#333')
    plt.title(f"Runtimes Across Days ({iterations} Iterations)", fontsize=18, color='#333')

    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['left'].set_visible(False)
    ax.spines['bottom'].set_color('#ccc')

    ax.set_axisbelow(True)
    ax.yaxis.grid(True, color='#ccc')
    ax.xaxis.grid(False)

    def autolabel(rects, labels, color):
        """Attach a text label above each bar in *rects*, displaying its height."""
        for i in range(len(rects)):
            rect = rects[i]
            label = labels[i]
            height = rect.get_height()

            minimum_height = 12
            rotation = 0 if height < minimum_height else 90
            font_size = 8 if height < minimum_height else 12
            y_origin = height if height < minimum_height else height / 2
            ax.annotate(format(label),
                        xy=(rect.get_x() + rect.get_width() / 2, y_origin),
                        xytext=(0, 3),  # 3 points vertical offset
                        textcoords="offset points",
                        fontsize=font_size,
                        color=color,
                        rotation=rotation,
                        ha='center', va='center')

    autolabel(rects1, [format_time(item) for item in part1_medians], "#238")
    autolabel(rects2, [format_time(item) for item in part2_medians], "#843")

    # Save the graph to a file
    plt.tight_layout()
    plt.savefig("executionTimesBar.png")
    plt.close()
    print("Bars saved as 'executionTimesBar.png'.")


main()
