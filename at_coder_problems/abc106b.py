n = int(input())
count = 0

for i in filter(lambda x : x % 2 == 1,range(1, n+1)):
    yakusu = 0
    for j in range(1,i+1):
        if i % j == 0:
            yakusu += 1
    if yakusu == 8:
        count += 1

print(count)
