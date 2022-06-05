#ifndef _STRING_H
#define _STRING_H	1

void *memchr(const void *str, int c, long n);

int memcmp(const void *str1, const void *str2, long n);

void *memcpy(void *dest, const void *src, long n);

void *memmove(void *dest, const void *src, long n);

void *memset(void *str, int c, long n);

char *strcat(char *dest, const char *src);

char *strncat(char *dest, const char *src, long n);

char *strchr(const char *str, int c);

int strcmp(const char *str1, const char *str2);

int strncmp(const char *str1, const char *str2, long n);

int strcoll(const char *str1, const char *str2);

char *strcpy(char *dest, const char *src);

char *strncpy(char *dest, const char *src, long n);

long strcspn(const char *str1, const char *str2);

char *strerror(int errnum);

long strlen(const char *str);

char *strpbrk(const char *str1, const char *str2);

char *strrchr(const char *str, int c);

long strspn(const char *str1, const char *str2);

char *strstr(const char *haystack, const char *needle);

char *strtok(char *str, const char *delim);

long strxfrm(char *dest, const char *src, long n);

#endif /* <string.h> included.  */