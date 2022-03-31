//
// Created by ChenXuzheng on 2022/3/31.
//

int whileLoop(){
    int a = 0;
    int b = 30;
    int c = 10;

    do {
        c = b - c;
    } while (c >= 0);

    while (a < b) {
        a += 1;
        b -= 1;

        if (b == 3)
            break;
    }

    return 0;
}

int forLoop(){
    int b=255;
    for(int i=0;i<b;i++){
        b-=1;
    }
    return 0;
}