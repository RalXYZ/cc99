//
// Created by ChenXuzheng on 2022/3/31.
//


int func1(int a,int b){
    return a+b;
}
int bar(int a, int b, int c) {
    int rst = 0;
    while (a < b) {
        rst <<= c;
        a += 1;
    }
    return rst;
}
int main(){
    func1(1,2);
    bar(1,2,3);
}

