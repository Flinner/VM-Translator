# VM to Hack Assembly Translator

This is part of [nand2tetris](https://nand2tetris.org), extending my
other project ([Flinner/HackAssembler: An Assembler written in
V](https://github.com/Flinner/HackAssembler)).

# Objective
Build a  VM translator, conforming to the VM Specification, and to
the Standard VM-on-Hack Mapping, [Project 07 |
nand2tetris](https://www.nand2tetris.org/project07) focusing on the
implementation of the VM language's stack arithmetic and memory access
commands.

# Usage
```bash
hackvmtranslator [file]
```
where file is the `.vm` file to be translated. Output is to `stdout`


# Specification
## Diagram
![image](https://user-images.githubusercontent.com/85732279/125663277-4a742557-8ba8-441d-957b-fc8b4e3a26fc.png)

## Arithmetic and Logical Commands
| Command | Return value (after popping the operand/s) | Comment                              |
|---------|--------------------------------------------|--------------------------------------|
| `add`   | `x + y`                                    | Integer addition (2’s complement)    |
| `sub`   | `x- y`                                     | Integer subtraction (2’s complement) |
| `neg`   | `- y`                                      | Arithmetic negation (2’s complement) |
| `eq`    | `true if x ¼ y, else false`                | Equality                             |
| `gt`    | `true if x > y, else false`                | Greater                              |
| `lt`    | `true if x < y, else false`                | Less than                            |
| `and`   | `x And y`                                  | Bit-wise                             |
| `or`    | `x Or y`                                   | Bit-wise                             |
| `not`   | `Not y`                                    | Bit-wise                             |

## Memory Access Commands
| Segment       | Purpose                                                                                                              | Comments                                                                                            |
|---------------|----------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------|
| `argument`    | Stores the function’s arguments.                                                                                     | Allocated dynamically by the VM implementation when the function is entered.                        |
| `local`       | Stores the function’s local variables.                                                                               | Allocated dynamically by the VM implementation and initialized to 0’s when the function is entered. |
| `static`      | Stores static variables shared by all functions in the same .vm file.                                                | Allocated by the VM imp. for each .vm file; shared by all functions in the .vm file.                |
| `constant`    | Pseudo-segment that holds all the constants in the range 0 . . . 32767.                                              | Emulated by the VM implementation; Seen by all the functions in the program.                        |
| `this` `that` | General-purpose segments. Can be made to correspond to different areas in the heap. Serve various programming needs. | Any VM function can use these segments to manipulate selected areas on the heap.                    |
| `pointer`     | A two-entry segment that holds the base addresses of the this and that segments.                                     | Aligning the this (or that) segment to the heap area beginning in that address.                     |
| `temp`        | Fixed eight-entry segment that holds temporary variables for general use.                                            | May be used by any VM function for any purpose. Shared by all functions in the program.             |

## Program Flow Commands
```java
label	symbol      // Label declaration
goto	symbol      // Unconditional branching
if-goto symbol      // Conditional branching
```

## Function Calling Commands
```java
function functionName nLocals   // Function declaration, specifying the
                                // number of the function’s local variables
call functionName nArgs	        // Function invocation, specifying the
                                // number of the function’s arguments
return                          // Transfer control back to the calling function
```

