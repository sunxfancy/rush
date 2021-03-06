### Rush编程语言 = Python的语法 + C++的效率 + Lisp的动态能力


##### 为什么需要它？

理由一：Rush支持类似Python的无花括号语法，并且兼容60%的C++语法，Rush自身可用标准C++编译器编译或者自我编译（自举）。

理由二：Rush既可编译运行又可解释运行，它以静态类型为主，没有GC，并且支持翻译为C++，可达到与C++接近的性能。

理由三：它以Lisp作为中间层，支持mixin、宏、元编程以及各种动态特性，并且同时支持call by name、call by need和call by value。

理由四：Rush的设计目标是简洁、快速、稳定，完全开源，它的源码结构比lua简单得多，但实现的功能不比lua少。

Rush的整体设计：

![github](https://github.com/roundsheep/rush/blob/gh-pages/doc/rush.png "github")

##### Rush编码风格1：（类似《算法导论》的伪代码）

		define ← =

		void insertion_sort(rstr& a):
			for j ← 1; j<a.count; j ← j+1
				key ← a[j]
				i ← j-1
				while i>=0 && a[i]>key
					a[i+1] ← a[i]
					i ← i-1
				a[i+1] ← key

##### Rush编码风格2：（类似python的无花括号风格）

		bool next_permutation<T>(rbuf<T>& v):
			if v.count<=1  
				return false  
			next=v.count-1  
			for  
				temp=next  
				next--  
				if v[next]<v[temp]  
					mid=v.count-1  
					for !(v[next]<v[mid])  
						mid--  
					swap<T>(v[next],v[mid])  
					reverse<T>(v,temp)  
					return true  
				if next==0  
					reverse<T>(v,0)  
					return false

##### Rush编码风格3：（Lisp风格，S表达式用逗号分隔）

		void main():
			int a
			int b
			[int,=,[a,1]]
			[int,=,[b,2]]
			[rf,print,[[int,+,[a,b]]]]
			
##### Rush编码风格4：（这是标准C++语法，本段代码可用VC++、G++或Rush进行编译）

		static rbool inherit_proc(tsh& sh,tclass& tci,int level=0)
		{
			if(level++>c_rs_deep)
				return false;
			if(tci.vfather.empty())
				return true;
			rbuf<tword> v;
			for(int i=0;i<tci.vfather.count();i++)
			{
				rstr cname=tci.vfather[i].vword.get(0).val;
				tclass* ptci=zfind::class_search(sh,cname);
				if(ptci==null)
				{
					ptci=zfind::classtl_search(sh,cname);
					if(ptci==null)
						return false;
				}
				if(!inherit_proc(sh,*ptci,level))
					return false;
				v+=ptci->vword;
			}
			v+=tci.vword;
			tci.vword=v;
			return true;
		}

<br/>
##### Rush的性能测试数据：

（Intel i5 3.3GHZ，使用命令 xxx -nasm ..\src\rush.cxx 自举解析约25000行代码生成一个7M的ASM文件，可以看到Rush与C++的比分是1/3，这是由于Rush本身使用了move语义和短字符串优化，但Rush本身又不支持这种优化，因此有理由相信真实的Rush运行性能可达到C++的1/1.5，与C++已经相当接近。同时可以看到Rush的编译速度非常快，不使用nasm的情况下可5秒完成自举）

![github](https://github.com/roundsheep/rush/blob/gh-pages/doc/benchmark.png "github")

<br/>
Rush支持多种运行方式，方法如下：

##### JIT：

1. cd到bin目录
2. 命令行敲入 rush -jit ..\src\example\test\1.rs

##### 解释运行：

1. cd到bin目录
2. 命令行敲入 rush ..\src\example\test\1.rs

##### 编译运行（以NASM为后端）：

1. cd到bin目录
2. 命令行敲入 rnasm ..\src\example\test\1.rs

##### 编译运行（翻译为C++并以G++为后端）：

1. cd到bin目录
2. 命令行敲入 gpp ..\src\example\test\1.rs

##### 编译运行（翻译为ASM再翻译成C++，处于测试阶段）：

1. cd到bin目录
2. 命令行输入 rcpp ..\src\example\test\53.rs

##### 二进制翻译（将EXE翻译为C再翻译为Rush再翻译为JS，目前仅支持简单代码）：

1. cd到bin目录
2. 使用G++编译标准C++代码，输入 mingw_build ..\ext\ida\test.cpp
3. 这时得到一个标准PE文件 ext\ida\test.cpp.exe
4. 下载并解压 IDA pro 6.6（161M） http://pan.baidu.com/s/1kTs8K1h
5. cd到IDA主目录
6. 命令行输入 idaq -A ..\rush\ext\ida\test.cpp.exe （取决于解压路径）
7. 按Ctrl+F5，等待数秒可以得到文件 ext\ida\test.cpp.c
8. cd到bin目录，命令行输入 rush -jit ..\ext\ida\trans.rs ..\ext\ida\test.cpp.c
9. 这时可得到文件 ext\ida\test.rs
10. 命令行输入 rush -js ..\ext\ida\test.rs
11. 双击 ext\ida\test.html

##### HTML5在线解释运行（目前仅支持chrome，emscripten对函数指针转换支持貌似有问题）：

1. 点击 http://roundsheep.github.io/rush/
2. 点击run按钮，稍等几秒会显示运行结果

##### HTML5运行（翻译为JS，不需要emscripten，目前仅支持chrome，处于测试阶段）：

1. cd到bin目录
2. 命令行输入 rush -js ..\src\example\test\53.rs
3. 双击 ..\src\example\test\53.html

##### Mac OS 或 Ubuntu 解释运行（处于测试阶段）：

1. 确保安装了clang（3.5或以上）或g++（4.8或以上）
2. cd到bin目录
3. 命令行输入 g++ ../src/rush.cxx -o rush -w -m32 -std=c++11
4. 命令行敲入 ./rush ../src/example/test/50.rs

##### IOS：

1. cd到bin目录
2. 命令行敲入 rush -gpp ..\src\example\test\1.rs
3. 将生成的src\example\test\1.cpp和ext\mingw\gpp.h两个文件导入xcode
4. 根据需要修改main函数，注释掉windows.h头文件

##### Android：（绿色集成包，不依赖其他环境）

1. 确保编译环境为64位windows
2. 下载一键安卓工具包并解压到一个无空格无中文的路径（1.1G） http://pan.baidu.com/s/1c0oc3Ws
3. 点击create_proj.bat
4. 输入工程名如test，等待命令窗口结束
5. 点击proj\test\build_cpp.bat
6. 等待几分钟命令窗口出现“请按任意键继续”
7. 按回车键并等待命令窗口结束
8. 成功后会得到proj\test\proj.android\bin\xxx.apk
9. 根据需要将Rush翻译得到的CPP文件包含进proj\test\Classes\HelloWorldScene.cpp

##### X64：

1. cd到bin目录
2. 命令行敲入 rush -gpp64 ..\src\example\64bit_test.rs
3. 将生成的src\example\test\64bit_test.cpp导入Visual Studio（或者使用64位的G++）
4. 选择x64
5. 按F7

##### 第三方IDE编辑代码：

1. 运行ext\ide\SciTE.exe
2. 点击File->Open
3. 选择src\example\test\1.rs，点击“打开”
4. 按F5运行程序（或者F7生成EXE）

<br/>
Visual Assist智能补全请看视频演示：

http://www.tudou.com/programs/view/40Ez3FuqE10/

<br/>
##### 重新将Rush编译为JS：

1. 确保安装了emscripten
2. 打开Emscripten Command Prompt
3. cd到Rush主目录
4. 命令行敲入 em++ -O3 src\rush.cxx -o rush.html -w --preload-file src -s TOTAL_MEMORY=156777216 -s EXPORTED_FUNCTIONS="['_js_main']"

##### 重新编译Rush源码：

1. 确保安装了 VS2012 update4 或者 VS2013
2. 打开src\proj\rush.sln
3. 选择Release模式（因为JIT不支持Debug）
4. 按F7，成功后会生成bin\rush.exe

##### 自举（以NASM为后端）：

1. 双击self_test_nasm.bat
2. 稍等几分钟后会生成bin\rush_nasm.exe（实际上Rush完成自举只需要5秒，瓶颈在NASM，据说Chez Scheme自举也是5秒）
3. 注意自举后仅NASM模式和GPP模式可用

##### 自举（以G++为后端）：

1. 双击self_test_gpp.bat
2. 稍等几分钟后会生成bin\rush_gpp.exe（使用命令gpp_build ..\src\rush.cxx可13秒自举，只不过自举后第二次再自举需要60秒，-O0优化）
3. 注意自举后仅NASM模式和GPP模式可用

##### 调试：

1. cd到bin目录
2. 命令行敲入 rush -gpp ..\src\example\test\1.rs
3. 使用Visual Studio或gdb调试src\example\test\1.cpp

##### 自动测试example下所有例子：

1. 双击bin\example_test.bat

<br/>


QQ交流群：34269848   

E-mail：287848066@qq.com
