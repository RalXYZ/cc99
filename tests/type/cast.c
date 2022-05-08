//
// Created by ChenXuzheng on 2022/3/31.
//

typedef int i;

int cast(){
    double a=10.0;
    double b=10.1f;


    int c=a as int;

    float e=1.2;
    float f=1.3f;

    c=e as i;

    void *g = (&c) as (void *);
}
