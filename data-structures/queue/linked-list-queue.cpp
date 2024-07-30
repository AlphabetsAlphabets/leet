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

        // Moves node at specified index to the front of the queue, pushing everything back.
        void moveToFront(int index) {
            if (index == 0) {
                // do nothing
                return;
            }

            Node<T>* current = front;
            Node<T>* previous;

            for (int i = 0; i < index - 1; i++) {
                previous = current;
                // 2 -> 3 -> 4 -> 5
                current = current->next;
            }

            // 3 -> 4 -> 5
            Node<T>* target = current->next;
            // 4 -> 5
            Node<T>* remaining = target->next;
            // 2 -> 4 -> 5
            current->next = remaining;
            
            // 3 -> 1 -> 2 -> 4 -> 5
            target->next = front;
            front = target;
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
    
    queue->enqueue(1);
    queue->enqueue(2);
    queue->enqueue(3);
    queue->enqueue(4);
    queue->enqueue(5);

    queue->printQueue();
    queue->moveToFront(1);
    queue->printQueue();

    return 0;
}