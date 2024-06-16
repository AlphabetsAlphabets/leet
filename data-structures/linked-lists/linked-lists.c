#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Node {
    int data;
    struct Node *next;
} typedef Node;

Node* new_node(int data) {
    Node *new = malloc(sizeof(Node));
    new->data = data;
    new->next = 0;

    return new;
}

struct LinkedList {
    Node *head;
} typedef LinkedList;

void print_list(LinkedList *list) {
    if (list->head == 0) {
        printf("[]\n");
        return;
    }

    char *data = 0;
    Node* current_node = list->head;

    sprintf(data, "%i", current_node->data);

    while (current_node->next != 0) {
        current_node = current_node->next;
        data += current_node->data;
    }

    return;
}

void append(LinkedList *list, int data) {
    Node* node = new_node(data);
    if (list->head == 0) {
        list->head = node;
        return;
    }

    Node* current_node = list->head;
    while (current_node->next != 0) {
        current_node = current_node->next;
    }

    current_node->next = node;

    return;
}

int main() {
    int data = 20;
    char value[20];
    printf("%c\n", sprintf(value, "%i", data));
    return 0;
}
