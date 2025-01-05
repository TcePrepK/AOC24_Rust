# Runs the day normally and returns the results
import re
import subprocess


# Runs the day normally and returns the results
def get_results(day_input):
    try:
        result = subprocess.run(
            ["cargo", "run", "-p", f"day{day_input}"],
            cwd="./", capture_output=True, text=True, check=True)
        output_text = result.stdout
        print(output_text)

        results = re.findall(r"\( (.*) \) -", output_text)
        return results[0], results[1]
    except subprocess.CalledProcessError as e:
        print(f"Error running binary:\n{e.stderr}")
        return None, None


def main():
    # Reset the result file
    open("results.txt", "w").close()

    # Run each day and store the results
    for day in range(1, 26):
        part1, part2 = get_results(day)

        with open(f"results.txt", "a") as f:
            f.write(f"Day {day}:\n |> {part1}\n |> {part2}\n")


main()
