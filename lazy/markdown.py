def generate_markdown_table(file_path: str) -> str:
    with open(file_path, 'r') as file:
        lines = file.readlines()

    # Split lines into cells by tab and trim whitespace
    rows = [line.strip().split('\t') for line in lines]

    if not rows:
        return ''

    # Determine column count based on the first row
    col_count = len(rows[0])

    # Generate header separator line
    separator = '| ' + ' | '.join(['---'] * col_count) + ' |'

    # Generate Markdown table lines
    markdown_lines = ['| ' + ' | '.join(row) + ' |' for row in rows]

    # Insert separator after the header row (first row)
    markdown_lines.insert(1, separator)

    # Combine all lines into a single Markdown table string
    return '\n'.join(markdown_lines)


def save_markdown_table(file_path: str, markdown_content: str):
    with open(file_path, 'w') as file:
        file.write(markdown_content)


def main():
    input_file = 'input.txt'  # Change this to your input file name
    output_file = 'output.md'  # Change this to your desired output file name

    markdown_table = generate_markdown_table(input_file)
    save_markdown_table(output_file, markdown_table)


if __name__ == '__main__':
    main()