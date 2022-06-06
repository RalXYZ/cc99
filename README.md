# cc99

*cc99* (not [cc98.org](https://cc98.org)) is a C-like language compiler, which is the final project of ZJU Compiler Principle course. It supports many of the C99  language syntax, and can compile source code to executable file, assembly language, abstract syntax tree (json style) and so on. 

cc99 can be used on Linux, Mac, even windows (unofficial support), anyone can build from source code or download binary file to have a try!

![cc99](https://raw.githubusercontent.com/RalXYZ/repo-pictures/main/cc99/cc99.png)  

## Supported Syntax

### Types
- void
- char, short, int, long, long long
- unsigned char, unsigned short, unsigned int, unsigned long, unsigned long long
- \_Bool
- float, double
- pointer (any basic type)
- array (any basic type, any dimension)
- function
- struct

### Statements
- compound (which means `{}`)
- if, else
- while, dowhile, for, break, continue
- return

### Expressions
- assignment: `=  +=  -=  *=  /=  %=  &=  |=  ^=  >>=  <<=`
- unary: `++a  --a  a++  a--  +a  -a  |a  ^a  *a  &a  sizeof(a)`
-  binary: `a+b a-b a*b a/b a%b a|b a^b a^b a>>b a<<b a&&b a||b a==b a!=b a<b a>b a<=b a>=b a,b`
- function call: `a(10,20,30)`
- type cast: `identifier as T`
- conditional: `a>10?1:0`
- sizeof: `sizeof(a), sizeof(int), sizeof(int*)`
- member of struct: `struct course c; c.name`
- array subscript: `int a[10]; a[0]`
- identifier: `int a`
- literal (any base type): `123, 123.123, 123l, "123", '1'`



## Get cc99

Before we start, make sure you have already installed [gcc](https://gcc.gnu.org/) or [clang](https://clang.llvm.org/), because cc99 need one the them to link object files. You can click href and install one of them.

There are three ways to get cc99:

1. Download from [releases](https://github.com/RalXYZ/cc99/releases). We just provide Linux (x86_64) and MacOS (Intel chip) version. As you know, they can cover almost all of develop situations.

2. Build with [Docker](https://www.docker.com/). We provide a `Dockerfile` at root directory wihich can build cc99. It includes dir, web-frontend, web-backend into a Ubuntu image. You can get your own image by following steps:

  ~~~bash
  git clone https://github.com/RalXYZ/cc99.git
  cd cc99
  docker build . -t cc99:latest
  # now this image contains all target files, you can use `docker cp` to copy them out or start a container!
  
  # start a container named cc99_all, bind port and mount volumes
  docker run --name cc99_all -p 6001:5001 -v ./data/runtime:/backend/runtime -d cc99:latest 
  
  # get executable file
  docker cp cc99_all:/backend/cc99  .
  # get include files
  docker cp cc99_all:/backend/include  .
  ~~~

3. Compile from source code. Here is a sample (**on ubuntu:20.04**):

  ~~~bash
  git clone https://github.com/RalXYZ/cc99.git
  cd cc99
  sed -i 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list && \
   apt-get update && apt install build-essential git curl libssl-dev wget libz-dev -y
  
  # install rust toolchains
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  
  # use tuna registry to speed up
  echo '[source.crates-io]\n\
  registry = "https://github.com/rust-lang/crates.io-index"\n\
  replace-with = "tuna"\n\
  [source.tuna]\n\
  registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' > /root/.cargo/config
  
  # add llvm registry 
  echo 'deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-13 main' >> /etc/apt/sources.list
  
  # optional
  curl -LO http://archive.ubuntu.com/ubuntu/pool/main/libf/libffi/libffi6_3.2.1-8_amd64.deb
  dpkg -i libffi6_3.2.1-8_amd64.deb
  
  # install llvm
  wget -O a https://apt.llvm.org/llvm-snapshot.gpg.key && apt-key add a
  apt update -y && apt upgrade -y && apt install -y llvm-13 llvm-13-dev libllvm13 llvm-13-runtime libclang-common-13-dev
  
  # a necessary ENV
  export LLVM_SYS_130_PREFIX=/usr
  
  # build it!
  cargo build --package cc99 --bin cc99 --release
  ~~~



## Usage

At first you need write a source code file (in cc99 syntax, we will call it `a.c` in the following description). Then you can simply run: 

~~~bash
cc99 a.c 
# or
cc99 a.c -o a

# you will get an executable file named `a`, just run it!
./a 
# happy coding!
~~~

cc99 supports many command line arguments, you can simply use `cc99 --help` to find out:

- without any extra options: executable file, using clang(default) or gcc to link
- `-c` or `--assemble`: Compile and assemble, but do not link
- `-S` or `--compile`: Compile only; do not assemble or link
- `-b` or `--bitcode`: Generate LLVM bitcode only
- `-p` or `--parse`: Preprocess and parse; do not compile, assemble or link
- `-V` or `--visual`: Convert stdin as code file and generate AST format to stdout
- `-E` or `--expand`: Preprocess only; do not parse, compile, assemble or link

In addition, we provide some useful options:

- `-O` or `--opt-level <OPT_LEVEL>` : Optimization level, from 0 to 3 [default: 0]. Like `gcc` or `clang `provided, even have a more aggressive strategy than `gcc` and `clang`
- `-i` or `--include`: Add directories `<dir>, <dir>, <dir>`(from left to right) to the list of directories, and cc99 will search for header files during preprocessing. Absolute paths are strongly recommended.

### Compile/Run Online, Visualize AST

We provide a [demo playground](https://cc99.raynor.top), you can play with your source code online and compile/run with a provided backend. Also you can observe the AST (abstract syntax tree) of yout source code.

`web-frontend` uses `react.js` and `antv`, `web-backend` uses `golang` and `gin`. These two modules can be found in `./web` directory.  



## Compare with C99 Standard

To be honest, cc99 can parse almost all of the standard C99 language syntax, but we cannot generate some of them to bitcode. What's more, there exists some differences between our language standard and C99 standard. It is strongly recommended to read this chapter before having a try. 

### Preprocessing

~~~c
#include <custome_header.h>
#ifdef _DEF_CC99_1
#define CC99 C99
#endif
typedef double REAL;
~~~

We use four preprocess steps:

  - First pass: Process all line continuation characters. Add a newline if there is no newline at the end of the file.

  - Second pass: Delete all comments.

  - Third pass: Process all preprocessing directives like `#include`, `#define`, `#if` etc.

  - Fourth pass: Merge adjacent string literals, 

    > E.g. char s[] = “\033[0m””Hello”;  =>  char s[] = “\033[0mHello”

We provide three simple **header files**, which can be found in `/inlcude` directory. These files can cover most situations and you can try them as you like, but don't forget to include them using `#include <stdio.h>` ! 

You can also add other C runtime functions to `/include` dir, all your need is add a function signature, but there are something you need to notice:

  - Support variable parameter, like `scanf` and `printf`
  - Support parameter qualifier, you can add any standard qualifiers like `const `, `atomic` and so on
  - Don't support `size_t`, you must change `size_t` to `long (8 bytes)`
  - Function must be contained in standard glibc runtime
  - Welcome to submit PR to add them! 

### Multidimensional Arrays and Multidimensional Pointers

~~~c
int a[10][20][30];
a[0][0][1] = 123;

int *p = a[0][0];
p++;
int **pp = &(p + 1);
pp = malloc(sizeof(int*) * 10);
pp[0] = malloc(sizeof(int) * 10);
pp[0][0] = 1;
~~~

You can use arrays and pointers as you like, since we already support most of arrays and pointer operations. But be attention:   

  - Don't convert multidimensional arrays directly into multidimensional pointers, it's illegal

  ~~~c
  int array[10][10];
  int **ptr = array; //inlegal!!
  ~~~

  - Best Practices for multidimensional pointer

  ~~~c
  #include <stdlib.h>
  int ***p;
  p = malloc(sizeof(int**) * 10);
  for(int i = 0; i < 10; i++){
      p[i] = malloc(sizeof(int*) * 10);
  }
  for(int i = 0; i < 10; i++){
      for(int j = 0; j < 10; j++){
          p[i][j] = malloc(sizeof(int) * 10);
      }
  }
  p[i][j][k] = 123;
  ~~~

### Struct Support (Partial)

You can define a struct in global scope, but can't define it in function!(maybe later we will support). Here is a usage:

~~~c
#include <stdlib.h>
struct course {
    char *name; 
    int credit;   
    char ***pre_course;
    int pre_status_num; 
    int *pre_course_num; 
    int grade; 
};
int main(){
    struct course c;
    c.name = malloc(sizeof(char) * 10);
    c.credit = 10;
      
    int credit = c.credit;
}
~~~

However, we don't support to **access variable from pointer** currently, which means the following code is illegal currently:  

~~~c
struct course c;
struct course *ptr = &c;
char *t = ptr->name; //inlegal! Don't support
~~~

And also, we don't support **struct initial list**

~~~c
struct course c = {"123",123,....}//inlegal! Don't support 
~~~

### Type Cast

In standard c syntax, it use truncation to deal it, like following

  ~~~c
  long long int a = 114514114514;
  int b = a; // lose information but allowed
  ~~~

But in cc99, every basic type has a rank, and we deny **implicit type cast** from a lower rank to a higher rank. Here is the rank table:

| Name                          | Rank |
| ----------------------------- | ---- |
| void                          | 0    |
| _Bool                         | 1    |
| char, unsigned char           | 2    |
| short, unsigned shot          | 3    |
| int, unsigned int             | 4    |
| long, unsigned long           | 5    |
| long long, unsigned long long | 6    |
| float                         | 7    |
| double                        | 8    |

You can use explicit type cast to convert high rank to low rank

  ~~~c
  double a = 123.123;
  float b = a as float; // we use `var as type` syntax
  ~~~

 Yes, we use `var as type` syntax for explicit type cast

  ~~~c
  double a = 123.123;
  float b = (float)a;   // illegal!
  float c = float(a);   // illegal!
  float d = a as float; // legal
  ~~~

### Function Hoisting and Global Variable Promotion

  ~~~c
  int main(){
      int s = sum(1, 2); // legal, all funcitons will hoist to top
      int e = d + 10;    // legal, all global variables will hoist to top
  }
  int sum(int a, int b){
      return a + b;
  }
  int d = 10;
  ~~~
