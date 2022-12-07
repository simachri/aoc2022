# Day 4

## Imlementation idea

  Represent the ranges as binary numbers. The bit count of the binary number is 
  determined by the maximum value of both range ends.

  Use bitwise operations to identify if a range is contained in another:
  ```
  00111000 left range
  01111100 right range
  -------- logical AND
  00111000 AND result

  00111000 left range
  00111000 AND result
  -------- logical XOR
  00000000 XOR result -> is == 0, thath is, left is fully contained in right

  01111100 right range
  00111000 AND result
  -------- logical XOR
  01000100 XOR result -> is != 0, that is, right is not contained in left
  ```
