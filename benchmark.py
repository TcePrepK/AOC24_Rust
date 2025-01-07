import re
import subprocess

import numpy as np
from matplotlib import pyplot as plt, ticker


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


# Function to format the change in a human-readable format
def format_change(change):
    if change > 0:
        return f"(⬆ {change:.2f}%)"
    else:
        return f"(⬇ {abs(change):.2f}%)"


# Benchmarks the day and returns the times calculated via Criterion
def run_bench(day_input):
    try:
        print("Benchmarking process might take a while...")
        result = subprocess.run(
            ["cargo", "bench", "-p", f"day{day_input}"],
            cwd="./", capture_output=True, text=True, check=True)
        output_text = result.stdout

        time_regex = r"time:\s+\[.*?\s([\d.]+ [a-zµ]*)\s.*?\]"
        both_parts = re.findall(time_regex, output_text, re.MULTILINE)
        times = [parse_time(part.replace(" ", "")) for part in both_parts]

        return times[0], times[1]
    except subprocess.CalledProcessError as e:
        print(f"Error running binary:\n{e.stderr}")
        return None, None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None, None


# Gets the previously recorded data from the README.md file
def get_previous_data():
    day_regex = fr"\|\s+\[Day.*\].*\|\s+([\d.]+[a-zµ]*).*\|\s+([\d.]+[a-zµ]*).*\|"
    with open("README.md") as f:
        text = f.read()
        search = re.findall(day_regex, text, re.MULTILINE)

        part1_times = []
        part2_times = []
        for group in search:
            part1_times.append(parse_time(group[0]))
            part2_times.append(parse_time(group[1]))
        return part1_times, part2_times


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
    day = int(input("Day: "))

    # Read the README.md and get every previous data
    part1_times, part2_times = get_previous_data()

    # Compile and get the benchmark results for this day
    cur_part1_time, cur_part2_time = run_bench(day)

    # Get the previous time data, then update it
    prev_part1_time = part1_times[day - 1]
    prev_part2_time = part2_times[day - 1]
    part1_times[day - 1] = cur_part1_time
    part2_times[day - 1] = cur_part2_time

    # Calculate the total time
    part1_total = sum(part1_times)
    part2_total = sum(part2_times)

    # Calculate the change in time
    part1_change = (cur_part1_time - prev_part1_time) / prev_part1_time * 100
    part2_change = (cur_part2_time - prev_part2_time) / prev_part2_time * 100

    print(f" |> {format_time(cur_part1_time)} {format_change(part1_change)}")
    print(f" |> {format_time(cur_part2_time)} {format_change(part2_change)}")
    print(f"Total Time:")
    print(f" |> {format_time(part1_total)} | {format_time(part2_total)}")

    write_to_readme = input("Write to README.md? (y/N): ")
    if write_to_readme.lower() == "y":
        print(f"Writing the changes to README.md!!!")
        readme_data = open("README.md", "r").read()

        # Replace previous day data with new day data
        previous_day_data = re.search(fr"\|\s+\[Day {day}].*(\|\s+[\d.]+[a-zµ]*.*\|\s+[\d.]+[a-zµ]*.*\|)",
                                      readme_data,
                                      re.MULTILINE).group(1)
        new_day_data = f"| {format_time(cur_part1_time)} {format_change(part1_change)} | {format_time(cur_part2_time)} {format_change(part2_change)} |"
        readme_data = readme_data.replace(previous_day_data, new_day_data)

        # Replace previous total time data with new total time data
        prev_total_time = re.search(
            r"\|\s+Total (\(.*\)\s+\|\s+[\d.]+[a-zµ]*.*\|\s+[\d.]+[a-zµ]*.*\|)",
            readme_data,
            re.MULTILINE).group(1)
        new_total_time = f"({format_time(part1_total + part2_total)}) | {format_time(part1_total)} | {format_time(part2_total)} |"
        readme_data = readme_data.replace(prev_total_time, new_total_time)

        with open("README.md", "w") as f:
            f.write(readme_data)

        print(f"Updated README.md")

        print()
        print("Creating the plots...")
        days = 25
        generate_bar_plot(days, part1_times, part2_times)
        generate_pie_chart(days, part1_times, part2_times)
    else:
        print(f"Doing nothing...")


main()
