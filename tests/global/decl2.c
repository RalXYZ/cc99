//
// Created by ralxyz on 5/15/22.
//

#define MAX_SIZE 10010
int data[10010];
// quick_sort by recursive 快速排序，使用递归

void quick_sort(int left, int right);
int main();

void quick_sort(int left, int right) {
    int pivot;
    int i;
    int j;
    if (left >= right) {
        return;
    }
    pivot = data[left];
    i = left;
    j = right;
    while (i < j) {
        while (i < j && data[j] >= pivot) {
            j -= 1;
        }
        data[i] = data[j];
        while (i < j && data[i] <= pivot) {
            i += 1;
        }
        data[j] = data[i];
    }
    data[i] = pivot;
    quick_sort(left, i - 1);
    quick_sort(i + 1, right);
    return;
}

int main(){
    int n;
    n = 1;
    quick_sort(0, n - 1);
    return 0;
}
