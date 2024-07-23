#include <iostream>

const int size = 100;

class Queue {
    public:
        int queue[size];
        int front, rear;

    Queue() {
        front = rear = -1;
    }

    bool isEmpty() {
        return rear == -1 && front == -1;
    }

    bool isFull() {
        return front == size;
    }

    bool enqueue(int value) {
        if (rear == size) {
            return false;
        }

        if (isEmpty()) {
            front++;
            rear++;
        } else {
            rear++;
        }

        queue[rear] = value;
        return true;
    }

    int dequeue() {
        if (isEmpty()) {
            return false;
        }

        return queue[front++];
    }
};

int main() {
    Queue queue;
    queue.enqueue(1);
    queue.enqueue(2);

    int value = queue.dequeue();
    std::cout << value << std::endl;

    value = queue.dequeue();
    std::cout << value << std::endl;

    return 0;
}