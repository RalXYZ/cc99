#include <stdio.h>
#include <string.h>
#include <stdlib.h>

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
    int *data;
    data=malloc(sizeof(int)*n);
    for(int i = 0; i < n; i++){
        scanf("%d", data+i);
    }
    quick_sort(data, 0, n - 1);
    for(int i = 0; i < n; i++){
        printf("%d\n", data[i]);
    }
    return 0;
}
