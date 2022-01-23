

class Person:
    def __init__(self, name: str):
        self.name = [x for x in name]
        self.len = len(self.name)

    def __iter__(self):
        self.n = 0
        return self


    def __next__(self):
        if self.len == self.n:
            raise Exception("end of iter")

        value = self.name[self.n]
        self.n += 1
        return value


person = iter(Person("Abdel"))

for char in person:
    print('char > ', char)

