a = int(input())
b = list(map(int, input().split()))

l = max(b)

ar = []
for v in b:
    ar.append(l - v)

nop = False
for i in range(1, a):
    if ar[i-1] < ar[i]:
        nop = True
        break

if nop:
    print(-1)
else:
    x = 0
    for i in range(a):
        y = ar[i+1] if i < a-1 else 0
        x += ar[i] - y
    print(x)
