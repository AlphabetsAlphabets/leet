class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

    def __repr__(self):
        return f"data: {self.data}, end: {self.next is None}"

class LinkedList:
    def __init__(self):
        self.len = 0
        self.head = None

    def __repr__(self):
        data = []
        current_node = self.head

        while current_node != None:
            data.append(f"{current_node.data}")
            current_node = current_node.next

        data = ", ".join(data)
        return f"[{data}]"

    def append(self, data):
        self.len += 1
        node = Node(data)
        if self.head == None:
            self.head = node
            return

        current_node = self.head
        while current_node.next != None:
            current_node = current_node.next

        current_node.next = node # type: ignore

    def prepend(self, data):
        if self.head == None:
            self.append(data)
            return

        right = self.head
        left = Node(data)
        left.next = right # type:ignore
        self.head = left

        self.len += 1

    def pop(self):
        if self.head == None:
            raise ValueError("Linked list is empty")

        current_node = self.head
        while current_node.next != None:
            current_node = current_node.next

            # Checks ahead of the next one
            # O      -> O           -> X
            # current   current.next   current.next.next
            if current_node.next.next != None:
                continue

            next = current_node.next
            data = next.data
            current_node.next = None
            self.len -= 1

            return data

    def insert(self, data, index):
        if self.head == None:
            self.append(data)
            return

        if index > self.len:
            raise IndexError("Index larger than length.")
        elif index == self.len:
            self.append(data)
            return

        current_node = self.head

        count = 0
        while count < index - 1:
            current_node = current_node.next # type: ignore
            count += 1
        
        original = current_node.next # type: ignore
        current_node.next = Node(data) # type: ignore
        current_node.next.next = original # type: ignore

    def search(self, data):
        if self.head == None:
            raise ValueError("Linked list is empty.")

        current_node = self.head
        len = 0
        while len != self.len:
            if current_node.data == data: # type: ignore
                return len - 1

            current_node = current_node.next # type: ignore
            len += 1

        return -1

    def index(self, index):
        if index < 0:
            raise IndexError("Index out of bounds.")

        len = 0
        current_node = self.head
        while len - 1 != index:
            current_node = current_node.next # type: ignore
            len += 1

        return current_node.data # type: ignore

list = LinkedList()
print(list)
list.append(1)
list.append(2)
list.append(3)
list.append(4)
list.insert(200, 4)
print(list)
