macro_rules! parse_input
{
    ( $x:expr, $t:ident ) => ( $x.trim().parse::< $t >().unwrap() )
}

fn input() -> String
{
    let mut input_line = String::new();
    std::io::stdin().read_line( &mut input_line ).unwrap();
    input_line
}

fn get_item< item >( input_value: &Option< item >) -> &item
{
    input_value.as_ref().map( |s| s ).unwrap()
}

#[ derive( Clone, Debug ) ]
struct water_set
{
    itself: Option< std::rc::Rc< std::cell::RefCell< water_set > > >,
    parent: Option< std::rc::Rc< std::cell::RefCell< water_set > > >,
    size:   usize
}

impl water_set
{
    fn new( mut input_parent: Option< std::rc::Rc< std::cell::RefCell< water_set > > > )
        -> std::rc::Rc< std::cell::RefCell< water_set > >
    {
        let mut return_set = water_set{ itself: None, parent: input_parent.clone(), size: 1 };
        let mut reference = std::rc::Rc::new( std::cell::RefCell::new( return_set ) );
        reference.borrow_mut().itself = Some( std::rc::Rc::clone( &reference ) );
        
        if let Some( ref mut set_parent ) = input_parent
        {
            set_parent.borrow_mut().size += 1;
        }
        else
        {
            reference.borrow_mut().parent = Some( std::rc::Rc::clone( &reference ) );
        }
        
        reference
    }
    
    /// This function sets the parent to be the highest parent in the set and returns it
    fn find( &mut self ) -> Option< std::rc::Rc< std::cell::RefCell< water_set > > >
    {
        if !std::rc::Rc::ptr_eq( get_item( &self.parent ), get_item( &self.itself ) )
        {
            let modify_value = get_item( &self.parent ).borrow_mut().find();
            self.parent = modify_value;
        }
        
        Some( std::rc::Rc::clone( get_item( &self.parent ) ) )
    }
}

#[ derive( Clone ) ]
enum location
{
    land,
    water( std::rc::Rc< std::cell::RefCell< water_set > > )
}

/// This program takes an input of a map of lakes and coordinates.
///  It outputs the total of connected water, if any, in relation
///  to each coordinate.
/// 
/// This version utilizes Disjoint set, which seems to be
///  significantly faster and uses less memory than flood fill.
fn main()
{
    let width = parse_input!( input(), u16 );
    let height = parse_input!( input(), u16 );
    
    let mut location_map = vec![ Vec::with_capacity( width as usize ); height as usize ];
    
    for count_y in 0..height as usize
    {
        let line = input().trim_right().to_string();
        eprintln!( "{}", line );
        
        for ( count_x, character ) in line.chars().enumerate()
        {
            if character == 'O'
            {
                let mut parent_up = None;
                let mut parent_left = None;
                
                if count_y > 0
                {
                    if let location::water( ref match_set ) = 
                    location_map[ ( count_y - 1 ) as usize ][ count_x as usize ]
                    {
                        parent_up = match_set.borrow_mut().find() ;
                    }
                }
                
                if let Some( location::water( ref match_set ) )
                    = location_map[ count_y as usize ].last_mut()
                {
                    parent_left = match_set.borrow_mut().find();
                }
                
                let mut new_parent = parent_up.clone();
                
                if parent_up.is_some()
                {
                    if let ( Some( set_up ), Some( set_left ) ) = ( parent_up, parent_left )
                    {
                        if !std::rc::Rc::ptr_eq( &set_up, &set_left )
                        { // they are not in same set, so we merge them
                            let mut merge_set1 = set_up.borrow_mut();
                            let mut merge_set2 = set_left.borrow_mut();
                            
                            if merge_set1.size < merge_set2.size
                            {
                                merge_set2.size += merge_set1.size;
                                merge_set1.parent = merge_set2.itself.clone();
                                new_parent = merge_set2.itself.clone();
                            }
                            else
                            {
                                merge_set1.size += merge_set2.size;
                                merge_set2.parent = merge_set1.itself.clone();
                                new_parent = merge_set1.itself.clone();
                            }
                        }
                    }
                }
                else
                {
                    new_parent = parent_left;
                }
                
                location_map[ count_y ].push( location::water(
                    water_set::new( new_parent ) ) );
            }
            else
            {
                location_map[ count_y ].push( location::land );
            }
        }
    }
    
    for _ in 0..parse_input!( input(), usize )
    {
        let input = input();
        let inputs = input.split( " " ).collect::< Vec < _ > >();
        let coordinate_X = parse_input!( inputs[ 0 ], u16 );
        let coordinate_Y = parse_input!( inputs[ 1 ], u16 );
        let temp;
        
        println!( "{}",
            match location_map[ coordinate_Y as usize ][ coordinate_X as usize ]
            {
                location::water( ref match_set )    =>
                    {
                        temp = match_set.borrow_mut().find();
                        get_item( &temp ).borrow_mut().size
                    },
                location::land                      => 0
            }
        );
    }
}
