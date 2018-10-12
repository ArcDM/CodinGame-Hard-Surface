#include <iostream>

/**This program takes an input of a map of lakes and coordinates.
 * It outputs the total of connected water, if any, in relation
 * to each coordinate.
 * 
 * This version utilizes Disjoint set, which seems to be
 * significantly faster and uses less memory than flood fill.
 **/

class Set
{
    private:
        Set* parent;
    
    public:
        size_t size;
    
        inline Set()
        {
            parent = this;
            size = 1;
        }
        
        Set* Find()
        { // This function sets the parent to be the highest parent in the set and returns it
            if( parent != this )
            {
                parent = parent->Find();
            }
            
            return parent;
        }
        
        inline void Union( Set* input_set )
        { // This function merges the two set if they have not been already
            if( Find() != input_set->Find() )   
            { // they are not in same set, so we merge them
                if( parent->size < input_set->parent->size )
                {
                    input_set->parent->size += parent->size;
                    parent->parent = input_set->parent;
                }
                else
                {
                    parent->size += input_set->parent->size;
                    input_set->parent->parent = parent;
                }
            }
        }
};

int main()
{
    int width, height, coordinate_count, coordinate_X, coordinate_Y;
    std::string location;

    std::cin >> width; std::cin.ignore();
    std::cin >> height; std::cin.ignore();
    
    Set* set_map[ height ][ width ] = {};

    for( size_t count_y = 0; count_y < height; ++count_y )
    {
        getline( std::cin, location );
        
        std::cerr << location << std::endl;
        
        for( size_t count_x = 0; count_x < width; ++count_x )
        {
            if( !location.compare( count_x, 1, "O" ) )
            {
                set_map[ count_y ][ count_x ] = new Set();
            
                if( count_y > 0 && set_map[ count_y - 1 ][ count_x ] )
                {
                    set_map[ count_y ][ count_x ]->Union( set_map[ count_y - 1 ][ count_x ] );
                }
            
                if( count_x > 0 && set_map[ count_y ][ count_x - 1 ] )
                {
                    set_map[ count_y ][ count_x ]->Union( set_map[ count_y ][ count_x - 1 ] );
                }
            }
        }
    }

    std::cin >> coordinate_count; std::cin.ignore();
    
    for( size_t count = coordinate_count; count; --count )
    {
        std::cin >> coordinate_X >> coordinate_Y; std::cin.ignore(); 
        
        if( set_map[ coordinate_Y ][ coordinate_X ] )
        {
            std::cout << set_map[ coordinate_Y ][ coordinate_X ]->Find()->size << std::endl;
        }
        else
        {
            std::cout << 0 << std::endl;
        }
    }
}