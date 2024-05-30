// Macros
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define SWAP(a, b) (a ^= b ^= a ^= b)

// Structure and union
struct Student {
    char name[50];
    int age;
    float gpa;
};

union Value {
    int intValue;
    float floatValue;
    char charValue;
};

// Function prototypes
void printArray(int arr[], int size);
int findMax(int arr[], int size);
void bubbleSort(int arr[], int size);
int isEven(int num);
int isOdd(int num);
int isPrime(int num);
int fibonacci(int n);

int main() {
    // Arrays
    int numbers[] = {5, 2, 8, 1, 9};
    int size = sizeof(numbers) / sizeof(numbers[0]);

    // Pointers
    int *ptr = numbers;
    printf("First element: %d\n", *ptr);
    printf("Second element: %d\n", *(ptr + 1));

    // Structure
    struct Student student = {"John Doe", 20, 3.8};
    printf("Student name: %s\n", student.name);
    printf("Student age: %d\n", student.age);
    printf("Student GPA: %.2f\n", student.gpa);

    // Union
    union Value value;
    value.intValue = 42;
    printf("Value (int): %d\n", value.intValue);
    value.floatValue = 3.14;
    printf("Value (float): %.2f\n", value.floatValue);
    value.charValue = 'A';
    printf("Value (char): %c\n", value.charValue);

    // Function calls
    printf("Unsorted array: ");
    printArray(numbers, size);
    bubbleSort(numbers, size);
    printf("Sorted array: ");
    printArray(numbers, size);
    printf("Maximum value: %d\n", findMax(numbers, size));

    // Control structures
    int num = 15;
    printf("%d is %s\n", num, isEven(num) ? "even" : "odd");
    printf("%d is %s\n", num, isPrime(num) ? "prime" : "composite");

    // Recursion
    int n = 10;
    printf("Fibonacci of %d: %d\n", n, fibonacci(n));

    return 0;
}

void printArray(int arr[], int size) {
    printf("[");
    for (int i = 0; i < size; i++) {
        printf("%d", arr[i]);
        if (i != size - 1) {
            printf(", ");
        }
    }
    printf("]\n");
}

int findMax(int arr[], int size) {
    int max = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

void bubbleSort(int arr[], int size) {
    for (int i = 0; i < size - 1; i++) {
        for (int j = 0; j < size - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                SWAP(arr[j], arr[j + 1]);
            }
        }
    }
}

int isEven(int num) {
    return num % 2 == 0;
}

int isOdd(int num) {
    return num % 2 != 0;
}

int isPrime(int num) {
    if (num <= 1) {
        return 0;
    }
    for (int i = 2; i <= sqrt(num); i++) {
        if (num % i == 0) {
            return 0;
        }
    }
    return 1;
}

int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}