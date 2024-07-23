/* lab9-queueArrayImpl.h */

#include <iostream>
#include "collection.h"

using namespace std;

#define SIZE 100

/* OOP: Inheritance - child class of the parent class */

class QueueArr : public Collection
{
public:
    int queue[SIZE];
    int front, rear;

public:
    QueueArr()
    {
        front = rear = -1;
    }

    bool isFull()
    {
        return (front == 0) && (rear == SIZE - 1);
    }

    bool isEmpty()
    {
        return front == -1;
    }

    void enqueue(int value)
    {
        if (isFull())
            cout << "Queue is full!" << endl;
        else
        {
            // When the queue is empty front and rear == -1
            // 1, 2, 3, 4
            // ^        ^
            // front    rear
            cout << "Enqueue element = " << value << endl;
            if (front == -1)
                front = 0;

            // rear++ makes -1 -> 0.
            rear++;
            queue[rear] = value;
        }
    }

    int dequeue()
    {
        int elem = -1;
        if (isEmpty())
            cout << "Queue is empty!" << endl;
        else
        {
            // 1, 2, 3, 4
            // ^        ^
            // front    rear
            elem = queue[front];
            if (front >= rear)
            {
                // This means there is only one element in the queue
                front = -1;
                rear = -1;
            }
            else
            {
                // Since 1 is gone, the front is now 2 so increment index.
                // X, 2, 3, 4
                //    ^     ^
                //    front rear
                // The problem with this is that each queue can only be used so many times.
                front++;
            }
            cout << "Deleted element = " << elem << endl;
        }
        return elem;
    }

    void show()
    {
        if (isEmpty())
            cout << "Queue is empty!" << endl;
        else
        {
            cout << "Queue elements [FIFO] = ";
            for (int i = front; i <= rear; ++i)
            {
                cout << queue[i] << " ";
            }
            cout << endl;
        }
    }

    int first()
    {
        return (!isEmpty()) ? queue[front] : -1;
    }

    int last()
    {
        return (!isEmpty()) ? queue[rear] : -1;
    }
};
