#include <stdio.h>
#include <stdlib.h>

typedef enum ValueType {
    ValueNumber,
    ValueString,
    ValueFunc
} ValueType;

typedef struct Value {
    union {
        double number;
        char *string;
        void (*func)();
    } data;

    ValueType type;
} Value;

void value_print(Value *value) {
    switch (value->type) {
        case ValueNumber: {
            printf("%d\n", value->data.number);
            break;
        }

        case ValueString: {
            puts(value->data.string);
            break;
        }

        case ValueFunc: {
            puts("<func>");
            break;
        }
    }
}

typedef struct Stack {
    Value *data;
    size_t length;
    size_t capacity;
} Stack;

Stack stack;

void stack_init() {
    Value *data = malloc(sizeof(Value) * 64);  // Arbitrary
    if (!data) {
        puts("Could not allocate stack");
        exit(1);
    }

    stack.data = data;
    stack.length = 0;
    stack.capacity = 64;
}

void stack_free() {
    free(stack.data);
}

int main() {
    stack_init(&stack);

    stack_free(&stack);
}