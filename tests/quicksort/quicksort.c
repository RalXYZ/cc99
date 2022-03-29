//
// Created by ChenXuzheng on 2022/3/28.
//

// 标准库中使用了__restrict关键词，限制Pointer aliasing，但是我们目前先不实现这个c99关键词？
extern int scanf (const char *__format, ...);
extern int printf (const char *__format, ...);

//TODO macro的支持
//int MAX_SIZE=10010;
int data[10010];
// quick_sort by recursive 快速排序，使用递归
void quick_sort(int *array, int left, int right) {
    if (left >= right) {
        return;
    }
    int pivot = array[left];
    int i = left;
    int j = right;
    while (i < j) {
        while (i < j && array[j] >= pivot) {
            j--;
        }
        array[i] = array[j];
        while (i < j && array[i] <= pivot) {
            i++;
        }
        array[j] = array[i];
    }
    array[i] = pivot;
    quick_sort(array, left, i - 1);
    quick_sort(array, i + 1, right);
}

int main(){
    int n;
    scanf("%d", &n);
    //TODO malloc要用吗？
    for(int i = 0; i < n; i++){
        scanf("%d", &data[i]);
    }
    quick_sort(data, 0, n - 1);
    for(int i = 0; i < n; i++){
        printf("%d\n", data[i]);
    }
    return 0;
}
