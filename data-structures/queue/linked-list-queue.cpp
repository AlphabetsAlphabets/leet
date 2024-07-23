#include <iostream>

template <typename T>
class Node {
    public:
        T value;
        Node* next;
};

template <typename T>
class Queue {
    public:
        Node<T>* front;

        Queue() {
            front = nullptr;
        }

        bool isEmpty() {
            return front == nullptr;
        }

        // Adds an element to the back of the queue
        void enqueue(T value) {
            Node<T>* newNode = new Node<T>();
            newNode->value = value;
            newNode->next = nullptr;

            if (isEmpty()) {
                front = newNode;
                return;
            } 

            Node<T>* current = front;
            if (current->next == nullptr) { // If there's only 1 element in the queue
                newNode->value = value;
                current->next = newNode;
                return;
            }

            // If there are more than 1 element
            while (current->next != nullptr) {
                current = current->next;
            }

            newNode->value = value;
            current->next = newNode;
        }

        // Removes the first element
        T dequeue() {
            if (isEmpty()) {
                std::cout << "Queue is empty." << std::endl;
                return -1;
            }

            Node<T>* remaining;
            int value = front->value;
            front = remaining;

            return value;
        }

        // Prints the queue. Left most element is the front.
        void printQueue() {
            Node<T>* current = front;

            if (isEmpty()) {
                std::cout << "Queue is empty" << std::endl;
                return;
            }

            std::cout << current->value << " ";

            while (current->next != nullptr) {
                current = current->next;
                std::cout << current->value << " ";
            }

            std::cout << std::endl;
        }
};

int main() {
    Queue<int>* queue = new Queue<int>();
    queue->dequeue();

    queue->enqueue(1);
    queue->enqueue(5);
    queue->enqueue(6);
    queue->enqueue(7);

    // 1 5 6 7
    queue->printQueue();

    queue->dequeue();
    queue->dequeue();
    queue->dequeue();

    // 7
    queue->printQueue();

    return 0;
}