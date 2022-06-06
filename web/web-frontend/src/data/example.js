export const ExampleCode = [
  {
    id: 0,
    name: "Basic",
    code: `int main() {
    return 0;
}`,
  },
  {
    id: 1,
    name: "Question 1",
    code: `#include <stdio.h>
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
        printf("%d\\n", data[i]);
    }
    return 0;
}
`,
  },
  {
    id: 2,
    name: "Question 2",
    code: `#include <stdio.h>
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
        printf("Incompatible Dimensions\\n");
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
        printf("\\n");
    }
    return 0;
}    `,
  },
  {
    id: 3,
    name: "Question 3",
    code: `#include <stdio.h>
#include <string.h>
#include <stdlib.h>
struct course {
    char *name; //名字，不超过5个字符
    int credit;   //学分
    char ***pre_course;
    int pre_status_num; //有多少个类
    int *pre_course_num; //每个类的数量
    int grade; //成绩
};

struct course courses[110];
int courses_num;

int try_credit, got_credit, remain_credit;

double gpa;
//查找某字符出现次数
int str_num(char *str, char c, long start, long end) {
    int ans=0;

    for (long i=start; i < end; i++)
    {
        if (str[i] == c)
            ans++;
    }
    return ans;
}

long find_char_in_range(char *str, char c, long start, long end)
{
    long i=start;
    for (; i < end; i++)
    {
        if (str[i] == c)
            return i;
    }
    return -1l;
}


int get_score(char flag)
{
    int score;
    if (flag == 'A')
        score = 4;
    else if (flag == 'B')
        score = 3;
    else if (flag == 'C')
        score = 2;
    else if (flag == 'D')
        score = 1;
    else if (flag == 'F')
        score = 0;
    else
        score = -1;
    return score;
}

int read_data()
{
    char buf[1000];
    while (scanf("%s", buf) != -1)
    {
        long index1 = find_char_in_range(buf, '|', 0, strlen(buf));
        long index2 = find_char_in_range(buf, '|', index1 + 1, strlen(buf));
        long index3 = find_char_in_range(buf, '|', index2 + 1, strlen(buf));

        courses[courses_num].name=malloc((index1+1)*sizeof(char));
        // printf("%d %d %d\\n", index1, index2, index3);
        strncpy(courses[courses_num].name, buf, index1);
        courses[courses_num].name[index1]='\\0';

        courses[courses_num].credit = buf[index1 + 1] - '0';

        courses[courses_num].grade = get_score(buf[index3 + 1]);

        if (courses[courses_num].grade > 0)
        {
            gpa += courses[courses_num].credit * courses[courses_num].grade;
        }

        if (courses[courses_num].grade >= 0)
        {
            try_credit += courses[courses_num].credit;
        }
        if (courses[courses_num].grade > 0)
        {
            got_credit += courses[courses_num].credit;
        }
        if (courses[courses_num].grade <= 0)
        {
            remain_credit += courses[courses_num].credit;
        }
        long semicolon = index2;
        long last_semicolon = index2;
        int pre_status_num = 0;

        if (index3 != index2 + 1)
        {
            int semicolon_num= str_num(buf,';',index2+1,index3);
            courses[courses_num].pre_course= malloc(sizeof(char**)*(semicolon_num+1));
            courses[courses_num].pre_course_num= malloc(sizeof(int)*(semicolon_num+1));
            // a trick, QAQ
            buf[index3] = ';';
            while ((semicolon = find_char_in_range(buf, ';', semicolon + 1, index3 + 1)) != -1)
            {
                int comma_num= str_num(buf,',',last_semicolon+1,semicolon);
                courses[courses_num].pre_course[pre_status_num]= malloc(sizeof(char*)*(comma_num+1));
                courses[courses_num].pre_course_num[pre_status_num]=comma_num+1;
                int pre_course_num = 0;
                long i;
                for (i = last_semicolon + 1; i < semicolon; i++)
                {
                    if (buf[i] == ',')
                    {
                        courses[courses_num].pre_course[pre_status_num][pre_course_num]= malloc(sizeof(char)*(i-last_semicolon));
                        strncpy(courses[courses_num].pre_course[pre_status_num][pre_course_num], buf + last_semicolon + 1,
                                i - last_semicolon - 1);
                        courses[courses_num].pre_course[pre_status_num][pre_course_num][i - last_semicolon-1]='\\0';
                        pre_course_num++;
                        last_semicolon = i;
                    }
                }
                courses[courses_num].pre_course[pre_status_num][pre_course_num]= malloc(sizeof(char)*(i-last_semicolon));
                strncpy(courses[courses_num].pre_course[pre_status_num][pre_course_num], buf + last_semicolon + 1,
                        i - last_semicolon - 1);
                courses[courses_num].pre_course[pre_status_num][pre_course_num][i-last_semicolon-1]='\\0';
                last_semicolon = semicolon;
                pre_status_num++;
            }
            buf[index3] = '|';
        }
        courses[courses_num].pre_status_num = pre_status_num;
        courses_num++;
    }
}

int main()
{
    read_data();
    if (try_credit > 0)
    {
        gpa /= try_credit;
    }
    printf("GPA: %.1lf\\n", gpa);
    printf("Hours Attempted: %d\\n", try_credit);
    printf("Hours Completed: %d\\n", got_credit);
    printf("Credits Remaining: %d\\n\\n", remain_credit);
    printf("Possible Courses to Take Next\\n");

    if (remain_credit == 0)
    {
        printf("  None - Congratulations!\\n");
        return 0;
    }
    int recommend_num = 0;
    for (int i = 0; i < courses_num; i++)
    {
        if (courses[i].grade <= 0)
        {
            if (courses[i].pre_status_num == 0)
            {
                printf("  %s\\n", courses[i].name);
                recommend_num++;
            }
            else
            {
                for (int j = 0; j < courses[i].pre_status_num; j++)
                {
                    int pre_num = 0;
                    int flag = 1;

                    for(int pre_num=0;pre_num<courses[i].pre_course_num[j];pre_num++){
                        int k;
                        for (k = 0; k < courses_num; k++)
                        {
                            if (strcmp(courses[i].pre_course[j][pre_num], courses[k].name) == 0)
                            {
                                if (courses[k].grade <= 0)
                                {
                                    flag = 0;
                                    break;
                                }
                                else
                                {
                                    break;
                                }
                            }
                        }
                        if (k == courses_num)
                        {
                            flag = 0;
                            break;
                        }
                    }
                    if (flag == 1)
                    {
                        printf("  %s\\n", courses[i].name);
                        recommend_num++;
                        break;
                    }
                }
            }
        }
    }
}   
      `,
  },

  {
    id: 4,
    name: "Unary",
    code: `int neg_unary() {
    int a = -2;
    return -a;
}

int not_unary() {
    int a = 1;
    return ~a;
}

int complement_unary() {
    int b = !0;
    int a = !23;
    return !b;
}
`,
  },
  {
    id: 5,
    name: "Binary",
    code: `int add() {
    int a = 2 + 2;
    return a;
}

int mul() {
    int a = 1 + 3 * 4 - 2;
    return a + 1;
}

int div() {
    int b = 2;
    int a = 3 / b;
    return b;
}`,
  },
  {
    id: 6,
    name: "If Condition",
    code: `int main() {
    int a = 0;
    int b = 1;

    if (a > b) {
        return 1;
    } else if (a < b) {
        return 2;
    } else {
        return 3;
    }
}`,
  },
  {
    id: 7,
    name: "For Loop",
    code: `int main() {
    int b = 255;

    for (int i = 0; i < b; i += 1) {
        b -= 1;
    }
    return 0;
}
`,
  },
  {
    id: 8,
    name: "While Loop",
    code: `int main() {
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
}`,
  },
  {
    id: 9,
    name: "Functions",
    code: `
int foo(int a, int b) {
    return a + b;
}

int main() {
    return foo(0, 1);
}

int bar(int a, int b, int c) {
    int rst = 0;
    while (a < b) {
        rst <<= c;
        a += 1;
    }
    return rst;
}
`,
  },
  {
    id: 10,
    name: "Global Variables",
    code: `int main() {
    a = 3;
    return a;
}

int a  = 0;

int foo() {
    return 0;
}
`,
  },
  {
    id: 11,
    name: "Type System",
    code: `int main() {
    bool a = false;
    byte b = 013;
    short c = 0x3;
    int d = 0b1011;
    long f = 0xcafebabe;

    float g = 0.5;
    double h = 0.25;

    return a;
}`,
  },
  {
    id: 12,
    name: "Cast Expression",
    code: `int cast() {

    double a = 10.5;

    int b = a as int;

    return b;
}`,
  },
  {
    id: 13,
    name: "String",
    code: `int main() {
    char* s = "String Test";

    printf("%s", s);

    return 0;
}`,
  },
];
