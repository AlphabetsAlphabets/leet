/* lab9-queueLinkedImpl.h */

#include <iostream>
#include "collection.h"

using namespace std;

class Node
{
public:
    int data;
    Node *next;
};

/* OOP: Inheritance - child class of the parent class */

class LinkedQueue : public Collection
{
    Node *front;
    Node *rear;

public:
    LinkedQueue()
    {
        front = nullptr;
        rear = nullptr;
    }

    // Enqueue relies on manipulating the rear node.
    // X --> X --> NULL
    // ^     ^
    // front rear
    //
    // X ---> X ---> X ---> NULL
    // ^             ^
    // front         rear
    // rear always points to the last entry.
    void enqueue(int value)
    {
        cout << "Enqueue element = " << value << endl;
        if (isEmpty())
        {
            rear = new Node();
            rear->data = value;
            rear->next = nullptr;
            front = rear;
        }
        else
        {
            Node *newNode = new Node();
            rear->next = newNode;
            newNode->data = value;
            newNode->next = nullptr;
            rear = newNode;
        }
    }


    int dequeue()
    {
        int elem = -1;
        Node *temp = front;
        if (front == nullptr)
        {
            cout << "Queue is underflow!" << endl;
        }
        else
        {
            elem = front->data;
            if (temp->next != nullptr)
            {
                // temp = X --> X --> NULL
                // front = X --> X --> NULL
                // temp->next = X --> NULL
                temp = temp->next;
                cout << "Deleted element = " << front->data << endl;
                // front = NULL
                free(front); // https://cplusplus.com/reference/cstdlib/free/
                // front = X --> NULL
                front = temp;
            }
            else // only 1 elem
            {
                // front = X --> NULL
                // rear = X --> NULL
                // they both point to the same thing.
                cout << "Deleted element = " << front->data << endl;
                // front = NULL
                free(front); // https://cplusplus.com/reference/cstdlib/free/
                // rear = NULL
                front = rear = nullptr;
            }
        } // outer-if
        return elem;
    }

    bool isEmpty()
    {
        return (front == nullptr) && (rear == nullptr);
    }

    void show()
    {
        Node *temp = front;
        if (isEmpty())
        {
            cout << "Queue is empty!" << endl;
        }
        cout << "Queue elements [FIFO] = ";
        while (temp != nullptr)
        {
            cout << temp->data << " ";
            temp = temp->next;
        }
        cout << endl;
    }

    int first()
    {
        return (!isEmpty()) ? front->data : -1;
    }

    int last()
    {
        return (!isEmpty()) ? rear->data : -1;
    }
};
