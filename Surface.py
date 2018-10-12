import sys

"""
    This program takes an input of a map of lakes and coordinates.
    It outputs the total of connected water, if any, in relation
    to each coordinate.
    
    This version utilizes Disjoint set, which seems to be
    significantly faster and uses less memory than flood fill.
"""

class Set:
    def __init__( this ):
        this.parent = this
        this.size = 1
        
    def Find( this ):
    # This function sets the parent to be the highest parent in the set and returns it
        if this.parent != this:
            this.parent = this.parent.Find()
            
        return this.parent
        
    def Union( this, input_set ):
    # This function merges the two set if they have not been already
        if this.Find() != input_set.Find(): # they are not in same set, so we merge them
            if this.parent.size < input_set.parent.size:
                input_set.parent.size += this.parent.size
                this.parent.parent = input_set.parent
            else:
                this.parent.size += input_set.parent.size
                input_set.parent.parent = this.parent
     
width = int( input() )
height = int( input() )
set_map = [ [ None ] * width for count in range( height ) ]

for count_y in range( height ):     
    for count_x, location in enumerate( input() ):
        print( location, end = "", file = sys.stderr )
    
        if location is "O":
            set_map[ count_y ][ count_x ] = Set()
            
            if count_y > 0 and set_map[ count_y - 1 ][ count_x ]:
                set_map[ count_y ][ count_x ].Union( set_map[ count_y - 1 ][ count_x ] )
            
            if count_x > 0 and set_map[ count_y ][ count_x - 1 ]:
                set_map[ count_y ][ count_x ].Union( set_map[ count_y ][ count_x - 1 ] )

    print( file = sys.stderr )

for count in range( int( input() ) ): # for the number of input coordinates
    coordinate_X, coordinate_Y = ( int( input_data ) for input_data in input().split() )
        
    if set_map[ coordinate_Y ][ coordinate_X ]:
        print( set_map[ coordinate_Y ][ coordinate_X ].Find().size )
    else:
        print( 0 )