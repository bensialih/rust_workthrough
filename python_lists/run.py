
names = ['abdel', 'mohammed', 'lucy', 'murad']

def change_x(value):
    return '%sx' % value

def filter_names(value: str):
    return value != "mohammedx"

# new_names = map(change_x, names)
# new_names = filter(filter_names, new_names)

# list_names = list(new_names)

# print(list_names, type(list_names))



class ListOfNames:
    names = []

    def __init__(self, names: list):
        self.names = names

    def map(self, change_func):
        self.names = map(change_func, self.names)
        return self

    def filter(self, filter_func):
        self.names = filter(filter_func, self.names)
        return self

    def collect(self):

        return list(self.names)


my_list = ListOfNames(names)
my_arrays = my_list.map(change_x).filter(filter_names).collect()
print('filtered and processed names ', my_arrays)
