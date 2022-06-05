# cc99

CC99 is a C-like language compiler ( with C99 Standard ) for ZJU Compiler Principle course. Not [cc98.org](https://cc98.org), we are compile of C99 ! It support almost all of the C99  language syntax, and can compile code to executable file, assembly language, ast (json style) and so on. 

CC99 can be used on Linux, Mac, even windows(Unofficial Support), anyone can build from source code or download binary file to have a try!



## Getting Started

At the begining of start, make sure you have already install [gcc](https://gcc.gnu.org/) or [clang](https://clang.llvm.org/), CC99 need one the them to link object files. You can click href and install one of them.

To install CC99, we provide three ways to choose:

- download from [releases](https://github.com/RalXYZ/cc99/releases), we just provide Linux(x86_64) and Mac(Intel chip) version. As you know, they can cover almost all of develop situations.

- compile with [Docker](https://www.docker.com/), we provide a `Dockerfile` at root directory, it can build CC99, include dir, Web-frontend, Web-Backend into a `ubuntu` image, you can get your own image by following steps:

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

- compile from source code, it maybe very difficult, here is a sample(**base ubuntu:20.04**):

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
  
  # add llvm registry to install 
  echo 'deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-13 main' >> /etc/apt/sources.list
  
  # optional
  curl -LO http://archive.ubuntu.com/ubuntu/pool/main/libf/libffi/libffi6_3.2.1-8_amd64.deb
  dpkg -i libffi6_3.2.1-8_amd64.deb
  
  # add gpg key
  wget -O a https://apt.llvm.org/llvm-snapshot.gpg.key && apt-key add a
  
  # install llvm
  apt update -y && apt upgrade -y && apt install -y llvm-13 llvm-13-dev libllvm13 llvm-13-runtime libclang-common-13-dev
  
  # a necessary ENV
  export LLVM_SYS_130_PREFIX=/usr
  
  # build it!
  cargo build --package cc99 --bin cc99 --release
  ~~~



## Compare with C99

To be honest, CC99 can parse almost all of the standard C99 language syntax, like 

- preprocess

  ~~~c
  #include <custome_header.h>
  #ifdef _DEF_CC99_1
  #define CC99 C99
  #endif
  typedef double REAL;
  ~~~

  we use four preprocess steps:

  - first pass:  Process all line continuation characters. Add a newline if there is no newline at the end of the file.

  - second pass: Delete all comments.

  - third pass: Process all preprocessing directives like `#include`, `#define`, `#if` etc.

  - fourth pass: Merge adjacent string literals, 

    > E.g. char s[] = “\033[0m””Hello”;  =>  char s[] = “\033[0mHello”

- Multidimensional Arrays, Multidimensional Pointers:

  ~~~c
  int a[10][20][30];
  a[0][0][1]=123;
  
  int *p=a[0][0];
  p++;
  int **pp=&(p+1);
  
  ~~~

  you can use arrays and pointers as you like, we already support most of arrays and pointer operations. But be attentions: don't 

