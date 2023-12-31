// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen
// by writing 'black' in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen by writing
// 'white' in every pixel;
// the screen should remain fully clear as long as no key is pressed.

//// Replace this comment with your code.
(LOOP)
@KBD
D=M

@PRINT
D; JNE

@LOOP
D; JEQ

(PRINT)
@i
M=0
(ILOOP)
@j
M=0
(JLOOP)
@16384
D=A
@i
D=D+M
@j
D=D+M
A=D
M=-1

@1
D=A
@j
M=M+D
D=M
@16
D=D-A
@JLOOP
D; JLT 

@16
D=A
@i
M=M+D
D=M
@8192
D=D-A
@ILOOP
D; JLT

(TMPLOOP)
@KBD
D=M
@TMPLOOP
D; JNE

@i
M=0
(ILOOP2)
@j
M=0
(JLOOP2)
@16384
D=A
@i
D=D+M
@j
D=D+M
A=D
M=0

@1
D=A
@j
M=M+D
D=M
@16
D=D-A
@JLOOP2
D; JLT 

@16
D=A
@i
M=M+D
D=M
@8192
D=D-A
@ILOOP2
D; JLT

@LOOP
0; JMP

