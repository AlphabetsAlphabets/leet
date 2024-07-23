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

        int value = queue[front];
        front++;
        return value;
    }

    void showQueue() {
        std::cout << front << std::endl;
        for (int i = front; i < rear; i++) {
            std::cout << queue[i] << " ";
        }

        std::cout << std::endl;
    }
};

int main() {
    Queue queue;
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(5);

    int value = queue.dequeue();
    value = queue.dequeue();
    
    queue.showQueue();

    return 0;
}