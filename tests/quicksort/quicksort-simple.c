//
// Created by TO/GA on 2022/5/20.
//

// 标准库中使用了__restrict关键词，限制Pointer
// aliasing，但是我们目前先不实现这个c99关键词？
extern int scanf(const char *__format, ...);
extern int printf(const char *__format, ...);

// TODO macro的支持
// int MAX_SIZE=10010;
int data[10010];
// quick_sort by recursive 快速排序，使用递归
void quick_sort(int left, int right) {
  if (left >= right) {
    return;
  }
  int pivot = data[left];
  int i = left;
  int j = right;
  while (i < j) {
    while (i < j && data[j] >= pivot) {
      j = j - 1;
    }
    data[i] = data[j];
    while (i < j && data[i] <= pivot) {
      i = i + 1;
    }
    data[j] = data[i];
  }
  data[i] = pivot;
  quick_sort(left, i - 1);
  quick_sort(i + 1, right);
}

int main() {
  int n;
  scanf("%d", &n);
  // TODO malloc要用吗？
  for (int i = 0, x; i < n; i = i + 1) {
    scanf("%d", &x);
    data[i] = x;
  }
  quick_sort(0, n - 1);
  for (int i = 0; i < n; i = i + 1) {
    printf("%d\n", data[i]);
  }
  return 0;
}
