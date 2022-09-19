
## Lynx Language Spec

### Basic Data Types

#### integer

```
let version = 1234;
```

#### string

```
let name = "Lynx programming language";
```

#### boolean

```
let is_cool = true;
```

#### hash

```
{
  "key": "foo",
  "value": "bar",
  1: ["arbitrary value"],
  2: {
    "child": ["arbitrary value"],
  },
}
```

#### array

```
let arr = [1, 2, 3, 4, 5];
let people = [{"name": "Anna", "age": 24}, {"name": "Bob", "age": 99}];
let arr_with_values = [1 + 1, 2 * 2, 3];
```

### operator

#### arithmetic operator

```
// addition
// subtraction
// multiplication
// division
(2 * 3) - (10 / 5) + 1;

let arithmeticValue = (10 / 2) * 5 + 30;
```

#### boolean logical operator

```
!true;
!false;
```

#### unary operator

```
+10;
-5;
"Foo" + " " + "Bar";
```

#### comparison operator

```
let isGreat = 2 > 5;
let isLess = 5 < 10;
let isEqual = 6 == 6;
```

### Flow of Control

#### If

```
if (true) {
  10;
} else {
  5;
}
```

#### While

```
while (true) {
  print("looping...");
}
```

#### Return

```
if (true) {
  return;
}

let return_stuff = fn(foo) {
  return foo;
};

return_stuff("Bar");
```

### Builtin Functions

```
len([0, 1, 2]); // 3
first([0, 1, 2]); // 0
last([0, 1, 2]); // 2
rest([0, 1, 2]); // [1, 2]
push([0, 1], 2); // [0, 1, 2]
unshift([0, 1], 2); // [2, 0, 1]
print("Hello Lynx"); // equivalent to console.log in JavaScript
```

## AST Definition

#### Expression

##### Integer

**Format:**

```
[-+]?[1-9][0-9]*;
```

**Example:**

```
1;
2345;
```

##### Boolean

**Format:**

```
true | false;
```

**Example:**

```
true;
let foo = false;
```

##### String

**Format:**

```
"<value>";
```

**Example:**

```
"foo_bar";
"foo" + " " + "bar"
```

##### Array

**Format:**

```
[
  <expression>, 
  <expression>, 
  ...
];
```

**Example:**

```
let array = [false, 1, fn(x) { x }];
array[0];
array[1];
array[2](10);
array[1 + 1](10);
```

##### Hashes

**Format:**

```
{ 
  <expression>: <expression>, 
  <expression>: <expression>, 
  ... 
};
```

**Example:**

```
let hash = {
  "name": "Foo",
  "age": 72,
  true: "a boolean",
  99: "an integer"
};

hash["name"];
hash["a" + "ge"];
hash[true];
hash[99];
hash[100 - 1];
```

##### Function

**Format:**

```
fn (<parameter one>, <parameter two>, ...) {
  <block statement> 
};
```

**Example:**

```
let add = fn(x, y) {
  return x + y;
};

add(10, 20);
```

// Expression
// Empty
// Ident(Token, String)
// NumberLiteral
// PrefixExpression
// InfixExpression
// BooleanLiteral
// If
// FunctionLiteral
// Call
// StringLiteral
// ArrayLiteral
// IndexExpression
// HashLiteral
// While  

```
// identifier
xyz 

// integer literal
5 

// bool literal
true/false 

// string literal
"abc" 

// array literal
[1,2,3] 

// hash literal
{ "a": 1 } 

// unary prefix expression
+1 

// binary infix expression
2 + 1 

// binary infix expression
x * y + 1
x <= y + 1 

// function literal
fn test(x) { x } 

// call expression
test(999); 

// if expression
if(x) { 1; } else { 0;} 

let fibonacci = fn(x) {
  if (x == 0) {
    0;
  } else {
    if (x == 1) {
      1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

// index expression
let arr = [0, 1, 2];
arr[1];
```

#### Statement

##### LetStatement

**Format:**

```
let <identifier> = <expression>;
```

**Example:**

```
let x = 0;
let y = 10;
let foobar = add(5, 5);
let alias = foobar;
let identity = fn(x) { x };
```

##### ReturnStatement

**Format:**

```
return <expression>;
```

**Example:**

```
return 1 + 2;
return foo;
return arr[1];
```

##### ExpressionStatement

**Format:**

```
<expression>;
```

**Example:**

```
1;
true;
"abc";
```

##### IfStatement

**Format:**

```
if (expression) {
  <statement | expression>
}
```

**Example:**

```
let bar = true;
if (bar) {
  print("bar");
}
```

##### WhileStatement

**Format:**

```
while (expression) {
  <statement | expression>
}
```

**Example:**

```
let foo = true;
while (foo) {
  print("foo");
}
```

#### Program

**Format:**

```
{
  <statement>;
  <statement>;
  ...
}
```

**Example:**

```
let x = 0; // let statement
let foobar = add(5, 5); // let statement
return x + y; // return statement

{ // block statement
  let x = 0; 
  let foobar = add(5, 5);
  return x + y;  
}
```

## Reference

### characters and punctuation marks

```
^ // caret
) // right parenthesis
` // backquote, grave accent
* // asterisk
! // bang, exclamation mark
[ ] // square brackets
( ) // brackets, parentheses
{ } // braces, curly brackets
<> // angle brackets
/ // forward slash
| // vertical bar
\ // backslash
- // hyphen
_ // underscore
"" // single or double quotation marks/ inverted commas
' // apostrophe
# // hash
^ // caret/circumflex
= // equal sign
== // double equal sign

(    open paren
)    close paren
[    open bracket  or open square bracket
]    close bracket or close square bracket
{    open curly    or open curly bracket
}    close curly   or close curly bracket
<    open angle    or open angle bracket   or less than
>    close angle   or close angle bracket  or greater than
|    pipe
"    double quote
'    single quote
:    colon
;    sem     or semicolon
!    bang    or not
^    hat     or caret
°    degree  or degrees or degree sign
#    pound   or number  or sharp  or hash sign
`    back tick
´    tick
§    section sign
-    hyphen  or minus
_    underline
~    twiddle or tilde
```