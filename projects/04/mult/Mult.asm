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
@R2
M=0

@sum
M=0

@i
M=0

(LOOP)
@i
M=M+1
D=M
@R0
D=M-D
@STOP
D; JLT

@R1
D=M
@sum
M=M+D
@LOOP
0; JMP
(STOP)
@sum
D=M
@R2
M=D
