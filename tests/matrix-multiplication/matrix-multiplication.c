//
// Created by ChenXuzheng on 2022/3/28.
//

// 标准库中使用了__restrict关键词，限制Pointer aliasing，但是我们目前先不实现这个c99关键词？
extern int scanf (const char *__format, ...);
extern int printf (const char *__format, ...);

//TODO 实现macro宏
//int MAX_ROW =25;
//int MAX_COL =25;

int a[25][25],b[25][25],c[25][25];
int a_row,a_col,b_row,b_col,c_row,c_col;

int main(){
    scanf("%d %d",&a_row,&a_col);
    for(int i=0;i<a_row;i++){
        for(int j=0;j<a_col;j++){
            scanf("%d",&a[i][j]);
        }
    }
    scanf("%d %d",&b_row,&b_col);
    for(int i=0;i<b_row;i++){
        for(int j=0;j<b_col;j++){
            scanf("%d",&b[i][j]);
        }
    }

    if (a_col != b_row) {
        printf("Incompatible Dimensions\n");
        return 0;
    }

    c_row = a_row;
    c_col = b_col;

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