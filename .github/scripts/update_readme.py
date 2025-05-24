import sys

def update_readme(help_file, readme_file, heading):
    with open(help_file, 'r') as help_f:
        help_output = help_f.read()

    with open(readme_file, 'r') as readme_f:
        lines = readme_f.readlines()

    # Find the heading and the start of the code block
    start_index = None
    end_index = None
    code_block_count = 0
    for i, line in enumerate(lines):
        if heading in line:
            start_index = i
        elif start_index is not None and line.strip() == "```":
            code_block_count += 1
            if code_block_count == 2:  # Second instance of "```"
                end_index = i + 1
                break

    if start_index is None or end_index is None:
        raise ValueError(f"Could not find the heading '{heading}' or its code block in the README.")

    # Replace the code block content
    new_lines = lines[:start_index + 1] + ["```\n"] + [help_output] + ["```\n"] + lines[end_index:]

    with open(readme_file, 'w') as readme_f:
        readme_f.writelines(new_lines)

if __name__ == "__main__":
    help_file = sys.argv[1]
    readme_file = sys.argv[2]
    heading = sys.argv[3]
    update_readme(help_file, readme_file, heading)