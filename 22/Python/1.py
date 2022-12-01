from heapq import heappush, heappop

heap = []
with open("inputs/1.txt", "r") as file:
    quant = 0
    for line in file:
        if line == "\n":
            heappush(heap, -quant)  # We have to push -quant so the heap acts like a MAX heap instead of a MIN heap.
            quant = 0
        else:
            quant += int(line.replace("\n", ""))

top_three = [heappop(heap) for _ in range(3)]
print(-top_three[0])
print(-sum(top_three))