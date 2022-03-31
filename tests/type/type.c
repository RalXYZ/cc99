//
// Created by ChenXuzheng on 2022/3/31.
//

int main(){
    short c = 0x3;
    int d = 0b1011;
    long f = 0xcafebabe;
    float g = 0.5;
    double h = 0.25;
    char i = 'a';
    char* s = "String Test";

    short*pc = &c;
    int*pd = &d;
    long*pf = &f;
    float*pg = &g;
    double*ph = &h;
    char*pi = &i;

    return 0;
}