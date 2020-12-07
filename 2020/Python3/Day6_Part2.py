f = open("input6.txt", "r")

sumYes = 0
numPeople = 0
answers = {

}

for line in f:
	if line != "\n":
		numPeople += 1
		for x in line:
			if x != "\n" and x not in answers:
				answers[x] = 1
			elif x != "\n":
				answers[x] = answers.get(x) + 1
	else:
		for a in answers:
			if answers[a] == numPeople:
				sumYes += 1
		numPeople = 0
		answers.clear()
print(sumYes)
