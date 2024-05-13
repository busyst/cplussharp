// Basic arithmetic and logic
int add(int a, int b) { return a + b; }
int subtract(int a, int b) { return a - b; }
int multiply(int a, int b) {
  int result = 0;
  for (int i = 0; i < b; i++) {
    result += a;
  }
  return result;
}
int divide(int a, int b) {
  if (b == 0) {
    PRINT("Error: Division by zero\n");
    return 0; // Handle division by zero
  }
  int result = 0;
  while (a >= b) {
    a -= b;
    result++;
  }
  return result;
}
int bitwise_and(int a, int b) { return a & b; }
int bitwise_or(int a, int b) { return a | b; }
int bitwise_not(int a) { return ~a; }
int bitwise_xor(int a, int b) { return a ^ b; }

// Control flow
void swap(int *a, int *b) {
  int temp = *a;
  *a = *b;
  *b = temp;
}
int max(int a, int b) {
  if (a > b) {
    return a;
  } else {
    return b;
  }
}
int min(int a, int b) {
  return max(b, a); // Leverage existing max function
}
void loop_example() {
  for (int i = 0; i < 10; i++) {
    PRINT("Iteration: ");
    PRINT(i);
    PRINT("\n");
  }

  int j = 0;
  while (j < 5) {
    PRINT("Looping: ");
    PRINT(j);
    PRINT("\n");
    j++;
  }

  int k = 1;
  do {
    PRINT("Do-while: ");
    PRINT(k);
    PRINT("\n");
    k++;
  } while (k < 3);
}

// Arrays (limited functionality without string manipulation)
void print_array_int(int arr[], int size) {
  for (int i = 0; i < size; i++) {
    PRINT(arr[i]);
    PRINT(" ");
  }
  PRINT("\n");
}

int sum_array(int arr[], int size) {
  int sum = 0;
  for (int i = 0; i < size; i++) {
    sum += arr[i];
  }
  return sum;
}

// Functions
void do_something(int x) {
  PRINT("Doing something with: ");
  PRINT(x);
  PRINT("\n");
}

// Limited memory management (error handling crucial)
void *my_malloc(int size) {
  // Implement system call for memory allocation (check for errors)
}
void my_free(void *ptr) {
  // Implement system call for memory deallocation
}

// Limited recursion (stack overflow risk)
int factorial(int n) {
  if (n == 0) {
    return 1;
  } else {
    return n * factorial(n - 1);
  }
}