# DiegOS
------
Basic OS for the x86-i386 architecture.
Works in 32-bit Protected Mode.  
Supports: 
- 4-KiB paging to address up to 40 MiB of physical and 4 GiB of virtual space,
- Global Descriptor Table which enables segmentation.
- Interrupt Handling,
- PIC driver which allows keyboard input,
- VGA Frame Buffer driver,
- Wrapper around Frame Buffer to create a standard println! macro interface,
- Error handling mechanisms and debugging tools,
- Memory Safety enforced by Rust, which saves from classical developing erros,
- Automatic build process using GNU Make.
	
## Installation:

In order to successfully build this project you must have the following software on your machine:

	GNU Binutils package for building rustup and custom target architecture compilation.
		Version: I used 2.4
	rustup:
		Insllation on https://doc.rust-lang.org/book/ch01-01-installation.html
		but on linux basically:
		$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
		
		Must also install nightly toolchain.
  		rustup toolchain install nigthly
	Bochs: features: --enable-debugger --with-sdl2
		Probably the Display Library could be changed but I tested with sdl2.
		Could be installed through package manager or through source at,
		https://bochs.sourceforge.io/doc/docbook/user/compiling.html
	GNU Make:
		To build project using a Makefile.

To compile this project just clone repository into a folder and then you can use any of the following make rules:     

	To compile and build project and create a HDD image readable by Bochs:
		$ mkdir build //Only the first time you are running make
  		$ make
  	Make has to be executed twice after clean.
	To clean build files:
		$ make clean
	To clean only HDD image:
		$ make clean-hdd
	To build empty HDD image of appropiate size:
		$ make hdd
	To run emulation	
		$ make dev
	Note: After Bochs loads up, you have to enter c into the command line to start execution.  

 
