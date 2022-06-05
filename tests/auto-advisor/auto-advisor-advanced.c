#include <stdio.h>
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
        // printf("%d %d %d\n", index1, index2, index3);
        strncpy(courses[courses_num].name, buf, index1);
        courses[courses_num].name[index1]='\0';

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
                        courses[courses_num].pre_course[pre_status_num][pre_course_num][i - last_semicolon-1]='\0';
                        pre_course_num++;
                        last_semicolon = i;
                    }
                }
                courses[courses_num].pre_course[pre_status_num][pre_course_num]= malloc(sizeof(char)*(i-last_semicolon));
                strncpy(courses[courses_num].pre_course[pre_status_num][pre_course_num], buf + last_semicolon + 1,
                        i - last_semicolon - 1);
                courses[courses_num].pre_course[pre_status_num][pre_course_num][i-last_semicolon-1]='\0';
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
    printf("GPA: %.1lf\n", gpa);
    printf("Hours Attempted: %d\n", try_credit);
    printf("Hours Completed: %d\n", got_credit);
    printf("Credits Remaining: %d\n\n", remain_credit);
    printf("Possible Courses to Take Next\n");

    if (remain_credit == 0)
    {
        printf("  None - Congratulations!\n");
        return 0;
    }
    int recommend_num = 0;
    for (int i = 0; i < courses_num; i++)
    {
        if (courses[i].grade <= 0)
        {
            if (courses[i].pre_status_num == 0)
            {
                printf("  %s\n", courses[i].name);
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
                        printf("  %s\n", courses[i].name);
                        recommend_num++;
                        break;
                    }
                }
            }
        }
    }
}