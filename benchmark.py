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
            [f"target/release/day{day_input}", "benchmark=100"],
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


# Generates the `results` file for storing and comparing results
def generate_results_file(days, part1_results, part2_results):
    # Open (or create) a file in the writing mode ('w')
    with open("results.txt", "w") as file:
        for day in range(0, days):
            file.write(f"Day {day + 1}:\n")
            file.write(f" |> {part1_results[day]}\n")
            file.write(f" |> {part2_results[day]}\n")
    print("Results saved as 'results.txt'.")


# Generates the bar plot image for the results
def generate_bar_plot(days, part1_medians, part2_medians):
    # Generate the labels and bar values of the graph
    labels = [f"Day {i}" for i in range(1, days + 1)]
    values_1 = [item * 1000 for item in part1_medians]
    values_2 = [item * 1000 for item in part2_medians]
    x = np.arange(len(labels))

    # Set up the whole graph
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
    plt.title(f"Runtimes Across Days", fontsize=18, color='#333')

    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.spines['left'].set_visible(False)
    ax.spines['bottom'].set_color('#ccc')

    ax.set_axisbelow(True)
    ax.yaxis.grid(True, color='#ccc')
    ax.xaxis.grid(False)

    r = fig.canvas.get_renderer()

    def autolabel(rects, labels, color):
        """Attach a text label above each bar in *rects*, displaying its height."""
        for i in range(len(rects)):
            rect = rects[i]
            label = labels[i]
            height = rect.get_window_extent(r).height
            time_height = rect.get_height()

            big_height = len(label) * 12
            rotation = 0 if big_height > height else 90
            font_size = 8 if big_height > height else 12
            y_origin = time_height if big_height > height else time_height / 2
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


# Generates the pie chart image for the results
def generate_pie_chart(days, part1_medians, part2_medians):
    # Calculate the percentages of each day
    day_medians = [part1_medians[i] + part2_medians[i] for i in range(days)]
    total_time = np.sum(day_medians)

    percentages = [day_medians[i] / total_time * 100 for i in range(days)]
    labels = [f"Day {i}" for i in range(1, days + 1)]

    # Create a color palette
    color_map = plt.get_cmap('turbo')
    wedge_colors = color_map(np.linspace(0.9, 0, len(labels)))

    # Reverse the percentages so that it rotates clockwise
    labels = labels[::-1]
    percentages = percentages[::-1]

    # Calculate the legend labels
    label_text = [f"{labels[i]} ({percentages[i]:.1f}%)" for i in range(len(labels))]

    # Do not render percentages below this threshold
    threshold_percentage = 3

    # Create the pie chart
    plt.figure(figsize=(12, 10))
    wedges, _, _ = plt.pie(
        percentages,
        autopct='',
        startangle=90,
        colors=wedge_colors,
        wedgeprops=dict(width=0.5, edgecolor='#ddd')
    )

    # Render the legend
    plt.legend(
        wedges, label_text, title="Days", loc="center left", bbox_to_anchor=(1, 0, 0.5, 1),
        fontsize=10, title_fontsize=12
    )

    # Add day labels inside the wedges
    for i, wedge in enumerate(wedges):
        # If the percentage is below the threshold, don't render the label
        if percentages[i] < threshold_percentage:
            continue

        # Calculate the mid-point of the wedge, then get position for the label
        angle = (wedge.theta2 + wedge.theta1) / 2
        x = np.cos(np.radians(angle)) * 0.75
        y = np.sin(np.radians(angle)) * 0.75

        # Adjust rotation to keep text readable
        rotation = angle if angle - 90 >= 180 else angle - 180
        plt.text(
            x, y, label_text[i], color=np.array([0, 0, 0, 0.3]), weight="bold", rotation=rotation,
            ha="center", va="center", rotation_mode="anchor"
        )

    # Title
    plt.title('Day-wise Percentage Distribution', fontsize=16, color='#000', weight='bold')

    # Save the graph to a file
    plt.tight_layout()
    plt.savefig("executionTimesPie.png")
    plt.close()
    print("Pie chart saved as 'executionTimesPie.png'.")


def main():
    # First, compile the Rust projects
    days = 25
    print("Compiling Rust projects...")
    for day in range(1, days + 1):
        compile_rust_project(day)

    start_time = time.time()

    print("Processing days...")
    part1_medians = []
    part2_medians = []
    part1_results = []
    part2_results = []

    # Run each day several times and store the results
    for day in range(1, days + 1):
        p1_times, p2_times, p1_result, p2_result = run_rust_binary(day)
        part1_medians.append(p1_times)
        part2_medians.append(p2_times)
        part1_results.append(p1_result)
        part2_results.append(p2_result)
    end_time = time.time()

    # Calculate the medians of the cleaned data
    total_median = sum(part1_medians) + sum(part2_medians)

    # Print the results
    for day in range(0, days):
        print(
            f"| [Day {day + 1}](./day{day + 1}/src/main.rs) | {format_time(part1_medians[day])} | {format_time(part2_medians[day])} |")
    print(f"Benchmarking process took {end_time - start_time:.2f} seconds.")
    print(f"Running each day once took {total_median:.2f} seconds.")

    print()

    generate_results_file(days, part1_results, part2_results)
    generate_bar_plot(days, part1_medians, part2_medians)
    generate_pie_chart(days, part1_medians, part2_medians)


main()
