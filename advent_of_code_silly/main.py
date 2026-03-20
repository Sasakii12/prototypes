# Advent of code 2022 Day 1
import input


def count_calories(cal: list[str]) -> list[int]:
    cal_sum: list[int] = []
    cal_sum_val = 0
    for i in cal:
        if i == '\n':
            cal_sum.append(cal_sum_val)
            cal_sum_val = 0
            continue
        cal_sum_val += int(i.strip())
    return cal_sum 

x = input.get_input("input.txt")
print(f"Cal max: {max(count_calories(x))}")
calories = count_calories(x)
top_three = sum([calories.pop(calories.index(max(calories))) for i in range(3)])
print(top_three)