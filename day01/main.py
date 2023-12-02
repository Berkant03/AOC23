data = list()

with open("data.txt", "r") as f:
    temp = list()
    for line in f:
        line = line.replace("\n","")
        if line == "----":
            data.append(temp.copy())
            temp.clear()
            continue
        temp.append(line)
data = data[1:]

data = [
    [v[0],int(v[1])] 
    for v in data
]

numbers = [
        "one", "two", "three",
        "four", "five", "six",
        "seven", "eight", "nine"
        ]

count = 0

for string, val in data:
    first_digit = 0
    second_digit = 0

    i = 0
    found = False
    while i < len(string) and not found:
        if string[i].isdigit():
            first_digit = int(string[i])
            break

        for x in range(len(numbers)):
            if string[i:].startswith(numbers[x]):
                found = True
                first_digit = x+1
                break

        i += 1

    j = len(string) - 1
    found = False
    while j >= 0 and not found:
        if string[j].isdigit():
            second_digit = int(string[j])
            break

        for x in range(len(numbers)):
            if string[:j+1].endswith(numbers[x]):
                found = True
                second_digit = x+1
                break

        j -= 1

    calibration = first_digit*10 + second_digit
    if calibration != val:
        print(f"{string}: {calibration}, {val}")
    count += calibration

# print(count)