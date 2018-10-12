# CodinGame-Hard-Surface

The goal of this puzzle:
    To find in a grid the list of adjacents cells that are from the same type (water or ground) and compute the size of the surface. The complexity of the problem is related to the size of the grid and the need to optimize the number of iterations. The cells will only be counted once in the traversal.

Input:
*   Line 1: the width L of the map  
*   Line 2: the height H of the map  
*   H following lines: L characters # or O  
*   Following line: the number N of coordinates to be tested  
*   N following lines: the coordinates X Y to be tested, separated by a space  

Output:
*   N lines: each displaying the surface area of the lake located at the coordinates given in input
