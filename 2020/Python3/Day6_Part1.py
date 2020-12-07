f = open("input6.txt", "r")

sumYes = 0

ar = []

for line in f:
	if line != "\n":
		for x in line:
			if x != "\n" and x not in ar:
				ar.append(x)
	else:
		sumYes += len(ar)
		print(ar)
		ar = []
print(sumYes)
