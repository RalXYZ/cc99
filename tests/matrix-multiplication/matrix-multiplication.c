#include <stdio.h>
#include <string.h>
#include <stdlib.h>


int **a,**b,**c;
int a_row,a_col,b_row,b_col,c_row,c_col;

int **init_matrix_ptr(int row,int col){
    int **tmp=malloc(sizeof(int*)*row);
    for(int i=0;i<row;i++){
        tmp[i]= malloc(sizeof(int)*col);
    }
    return tmp;
}

int main(){
    scanf("%d %d",&a_row,&a_col);
    a= init_matrix_ptr(a_row,a_col);
    for(int i=0;i<a_row;i++){
        for(int j=0;j<a_col;j++){
            scanf("%d",a[i]+j);
        }
    }
    scanf("%d %d",&b_row,&b_col);
    b=init_matrix_ptr(b_row,b_col);
    for(int i=0;i<b_row;i++){
        for(int j=0;j<b_col;j++){
            scanf("%d",b[i]+j);
        }
    }

    if (a_col != b_row) {
        printf("Incompatible Dimensions\n");
        return 0;
    }

    c_row = a_row;
    c_col = b_col;
    c=init_matrix_ptr(c_row,c_col);
    for(int i=0;i<c_row;i++){
        for(int j=0;j<c_col;j++){
            c[i][j] = 0;
            for(int k=0;k<a_col;k++){
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    for(int i=0;i<c_row;i++){
        for(int j=0;j<c_col;j++){
            printf("%10d",c[i][j]);
        }
        printf("\n");
    }
    return 0;
}