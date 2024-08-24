#include <stdio.h>
#include <stdlib.h>

typedef int VecItem;

typedef struct {
    int capacity;
    int len;
    VecItem* items;
} Vector;

Vector* new_vec() {
    Vector* vec = (Vector*)malloc(sizeof(Vector));

    if (vec == NULL) {
        exit(1);
    }

    vec->len = 0;
    vec->capacity = 4;
    vec->items = (VecItem*)malloc(vec->capacity * sizeof(VecItem));

    if (vec->items == NULL) {
        exit(1);
    }

    return vec;
}

int get_len(Vector* vec) {
    return vec->len;
}

int get_capacity(Vector* vec) {
    return vec->capacity;
}

int push(Vector* vec, int item) {
    if (vec->len == vec->capacity) {
        vec->capacity = vec->capacity * 2;
        int* new_items = (int*)realloc(vec->items, sizeof(Vector) * vec->capacity);

        if (new_items == NULL) {
            exit(1);
        }

        vec->items = new_items;
    }

    vec->items[vec->len] = item;
    vec->len++;
    return 0;
}

void free_vec(Vector* vec) {
    free(vec->items);
    free(vec);
}

void dissasemble_vec(Vector* vec) {
    printf("Len: %d \n", vec->len);
    printf("Capacity: %d \n", vec->capacity);
    printf("Items:\n");
    for (int i = 0; i < vec->len; i++) {
        printf("%d = %d\n", i, vec->items[i]);
    }
}

struct Class {
    char** students;
    int student_count;
};

int add_student(struct Class* class, char* student) {
    class->students[class->student_count] = student;
    class->student_count++;
    return 0;
}

void dissasemble_class(struct Class class) {
    for (int i = 0; i < class.student_count; i++) {
        printf("%s\n", class.students[i]);
    };
}

int main() {
    Vector* vec = new_vec();
    push(vec, 0);
    push(vec, 1);
    push(vec, 2);
    push(vec, 3);
    push(vec, 4);

    dissasemble_vec(vec);

    free_vec(vec);

    struct Class class;
    class.student_count = 0;
    class.students = (char**)malloc(4 * sizeof(char*));
    add_student(&class, "Tobias");
    add_student(&class, "Hello");
    add_student(&class, "TobiasWorld");

    dissasemble_class(class);

    return 0;
}

/*

struct Person { 
    age Int
}

impl Person {
    pub fn new(age Int) Self {
        Self {
            age: 
        }
    }
}

fn main() Int {

}

*/

/*

struct Person age Int end

main()

    

end

*/