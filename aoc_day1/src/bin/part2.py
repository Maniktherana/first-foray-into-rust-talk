# open ./input2.txt and store it in a list
with open("input2.txt") as f:
    content = f.readlines()
content = [x.strip() for x in content]

num_map = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}
res = []
for line in content:
    tmp = []
    for i in range(len(line)):
        if line[i].isdigit():
            tmp.append(int(line[i]))
        for j in range(i + 1, len(line)):
            if line[i : j + 1] in num_map:
                tmp.append(num_map[line[i : j + 1]])

    res.append(tmp)
ans = 0
for line in res:
    num = line[0] * 10 + line[-1]
    ans += num
print(ans)
