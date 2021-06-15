ans = 0

for i in range(1, 100):
    for j in range(1, 100):
        n = str(i ** j)
        s = 0
        for c in n:
            s = s + int(c)
        ans = max(ans, s)

print(ans)
