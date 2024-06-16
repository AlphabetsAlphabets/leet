class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None

    def __repr__(self):
        return f"data: {self.data}, left: {self.left is None}, right: {self.right is None}"

class LinkedList:
    def __init__(self):
        self.len = 0
        self.head = None

    def __repr__(self):
        current = self.head
        while current.right != None:
            value = current.data

        return ""

    def append(self, data):
        if self.head == None:
            self.head = Node(data)

        current = self.head
        while current.right != None:
            current = current.right

        current.next = Node(data)

    def prepend(self, data):
        if self.head == None:
            self.append(data)

        current = self.head
        right = current.right
        right.left = Node(data)
        self.head = right
