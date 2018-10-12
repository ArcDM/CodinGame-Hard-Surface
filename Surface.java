import java.util.Scanner;

/** This program takes an input of a map of lakes and coordinates.
 *   It outputs the total of connected water, if any, in relation
 *   to each coordinate.
 * 
 *  This version utilizes Disjoint set, which seems to be
 *   significantly faster and uses less memory than flood fill.
 **/

class Set
{
    private Set parent;
    public int size;
    
    public Set( final Set input_parent )
    {
        if( input_parent != null )
        {
            this.parent = input_parent;
            input_parent.size += 1;
        }
        else
        {
            this.parent = this;
        }

        this.size = 1;
    }

    public Set Find()
    { // This function sets the parent to be the highest parent in the set and returns it
        if( this.parent != this )
        {
            this.parent = this.parent.Find();
        }
            
        return this.parent;
    }
        
    public Set Union( Set input_set )
    { // This function merges the two set if they have not been already
        if( this.Find() != input_set.Find() )   
        { // they are not in same set, so we merge them
            if( this.parent.size < input_set.parent.size )
            {
                input_set.parent.size += this.parent.size;
                this.parent.parent = input_set.parent;
                return input_set.parent;
            }
            else
            {
                this.parent.size += input_set.parent.size;
                input_set.parent.parent = this.parent;
            }
        }

        return this.parent;
    }
}

class Solution
{
    public static void main( String arguments[] )
    {
        Scanner in = new Scanner( System.in );
        int width = in.nextInt();
        int height = in.nextInt();
        
        Set[][] set_map = new Set[ height ][];
        
        if( in.hasNextLine() )
        {
            in.nextLine();
        }
        
        for( int count_y = 0; count_y < height; ++count_y )
        {
            String location = in.nextLine();

            System.err.println( location );

            set_map[ count_y ] = new Set[ width ];
        
            for( int count_x = 0; count_x < width; ++count_x )
            {
                if( location.charAt( count_x ) == 'O' )
                {
                    Set new_parent = null;
            
                    if( count_y > 0 && set_map[ count_y - 1 ][ count_x ] != null )
                    {
                        if( count_x > 0 && set_map[ count_y ][ count_x - 1 ] != null )
                        {
                            new_parent = set_map[ count_y ][ count_x - 1 ].
                                Union( set_map[ count_y - 1 ][ count_x ] );
                        }
                        else
                        {
                            new_parent = set_map[ count_y - 1 ][ count_x ].Find();
                        }
                    }
                    else
                    {
                        if( count_x > 0 && set_map[ count_y ][ count_x - 1 ] != null )
                        {
                            new_parent = set_map[ count_y ][ count_x - 1 ].Find();
                        }
                    }
                
                    set_map[ count_y ][ count_x ] = new Set( new_parent );
                }
                else
                {
                    set_map[ count_y ][ count_x ] = null;
                }
            }
        }
        
        int coordinate_count = in.nextInt();
        
        for( int count = coordinate_count; count > 0; --count )
        {
            int coordinate_X = in.nextInt();
            int coordinate_Y = in.nextInt();

            if( set_map[ coordinate_Y ][ coordinate_X ] != null )
            {
                System.out.println( set_map[ coordinate_Y ][ coordinate_X ].Find().size );
            }
            else
            {
                System.out.println( 0 );
            }
        }
    }
}
