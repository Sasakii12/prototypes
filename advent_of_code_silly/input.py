def get_input(filename: str) -> list[str]:
    with open(filename) as file:
        lines: list[str] = []
        for line in file:
            lines.append(line)
        
        return lines