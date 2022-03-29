//
// Created by ChenXuzheng on 2022/3/28.
//

// 标准库中使用了__restrict关键词，限制Pointer aliasing，但是我们目前先不实现这个c99关键词？
extern int scanf(const char *__format, ...);
extern int printf(const char *__format, ...);

// struct course
// {
//     char name[5]; //名字，不超过5个字符
//     int credit;   //学分
//     char pre_course[8][8][5];
//     int pre_course_num;

//     int grade; //成绩
// };

// struct course courses[110];

char courses_name[110][5];
int courses_credit[110];
char courses_pre_course[110][8][8][5];
int courses_pre_course_num[110];
int courses_grade[110];

int courses_num;

int try_credit, got_credit, remain_credit;

double gpa;

int find_char(char *str, char c)
{
    int i;
    for (i = 0; str[i] != '\0'; i++)
    {
        if (str[i] == c)
            return i;
    }
    return -1;
}

int find_char_in_range(char *str, char c, int start, int end)
{
    int i;
    for (i = start; i < end; i++)
    {
        if (str[i] == c)
            return i;
    }
    return -1;
}

int strcpy_my(char *dest, char *src, int n)
{
    int i;
    for (i = 0; i < n; i++)
    {
        dest[i] = src[i];
    }
    dest[i] = '\0';
    return i;
}
int strlen_my(char *str)
{
    int i;
    for (i = 0; str[i] != '\0'; i++)
        ;
    return i;
}
int strcmp_my(char *str1, char *str2)
{
    int i;
    for (i = 0; str1[i] != '\0' && str2[i] != '\0'; i++)
    {
        if (str1[i] != str2[i])
            return str1[i] - str2[i];
    }
    return str1[i] - str2[i];
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
        int index1 = find_char_in_range(buf, '|', 0, strlen_my(buf));
        int index2 = find_char_in_range(buf, '|', index1 + 1, strlen_my(buf));
        int index3 = find_char_in_range(buf, '|', index2 + 1, strlen_my(buf));

        // printf("%d %d %d\n", index1, index2, index3);
        strcpy_my(courses_name[courses_num], buf, index1);
        courses_credit[courses_num] = buf[index1 + 1] - '0';

        courses_grade[courses_num] = get_score(buf[index3 + 1]);

        if (courses_grade[courses_num] > 0)
        {
            gpa += courses_credit[courses_num] * courses_grade[courses_num];
        }

        if (courses_grade[courses_num] >= 0)
        {
            try_credit += courses_credit[courses_num];
        }
        if (courses_grade[courses_num] > 0)
        {
            got_credit += courses_credit[courses_num];
        }
        if (courses_grade[courses_num] <= 0)
        {
            remain_credit += courses_credit[courses_num];
        }
        int semicolon = index2;
        int last_semicolon = index2;
        int pre_status_num = 0;

        if (index3 != index2 + 1)
        {
            // a trick, QAQ
            buf[index3] = ';';
            while ((semicolon = find_char_in_range(buf, ';', semicolon + 1, index3 + 1)) != -1)
            {
                int pre_course_num = 0;
                int i;
                for (i = last_semicolon + 1; i < semicolon; i++)
                {
                    if (buf[i] == ',')
                    {
                        strcpy_my(courses_pre_course[courses_num][pre_status_num][pre_course_num], buf + last_semicolon + 1,
                                  i - last_semicolon - 1);
                        pre_course_num++;
                        last_semicolon = i;
                    }
                }
                strcpy_my(courses_pre_course[courses_num][pre_status_num][pre_course_num], buf + last_semicolon + 1,
                          i - last_semicolon - 1);

                last_semicolon = semicolon;
                pre_status_num++;
            }
            buf[index3] = '|';
        }
        courses_pre_course_num[courses_num] = pre_status_num;
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
        if (courses_grade[i] <= 0)
        {
            if (courses_pre_course_num[i] == 0)
            {
                printf("  %s\n", courses_name[i]);
                recommend_num++;
            }
            else
            {
                for (int j = 0; j < courses_pre_course_num[i]; j++)
                {
                    int pre_num = 0;
                    int flag = 1;
                    while (courses_pre_course[i][j][pre_num][0] != '\0')
                    {
                        int k;
                        for (k = 0; k < courses_num; k++)
                        {
                            if (strcmp_my(courses_pre_course[i][j][pre_num], courses_name[k]) == 0)
                            {
                                if (courses_grade[k] <= 0)
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
                        pre_num++;
                    }
                    if (flag == 1)
                    {
                        printf("  %s\n", courses_name[i]);
                        recommend_num++;
                        break;
                    }
                }
            }
        }
    }
}