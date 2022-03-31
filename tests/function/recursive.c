//
// Created by ChenXuzheng on 2022/3/31.
//


void hannuo(int n,char a,char b,char c)
{
    if(n==1)
        printf("\t%c->%c\n",a,c);
    else
    {
        hannuo(n-1,a,c,b);
        printf("\t%c->%c\n",a,c);
        hannuo(n-1,b,a,c);
    }
}

int main() {
    int n;
    printf("请输入要移动的块数：");
    scanf("%d", &n);
    move(n, 'a', 'b', 'c');
    return 0;
}