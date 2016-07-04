extern crate nalgebra as na;

///Creates an scad module with optional children with the following syntax:
///scad!(parent);
///
///or
///
///scad!(parent;{child1 ... });
#[macro_export]
macro_rules! scad {
    ([ $parent:expr ]) => {$parent};

    ($parent:expr) => {ScadObject::new($parent)};

    //Parent followed by children in curly brakcets
    ($parent:expr;{$($child:expr),*$(),+}) => {
        {
            let mut tmp_stmt = ScadObject::new($parent);

            $(
                tmp_stmt.add_child($child);
            )*

            tmp_stmt
        }
    };

    //Recursive version of the macro
    ($parent:ident($($parent_params:expr)*){$($inner:tt)*}) => {
        {
            let mut tmp_stmt = ScadObject::new($parent($($parent_params),*));

            $(
                tmp_stmt.add_child(scad!($inner));
            )*

            tmp_stmt
        }
    };

    //Parent followed by children without curly brackets
    ($parent:expr;$($child:expr),*) => {
        {
            let mut tmp_stmt = ScadObject::new($parent);

            $(
                tmp_stmt.add_child($child);
            )*

            tmp_stmt
        }
    };
}

pub fn vec3(x: f32, y: f32, z:f32) -> na::Vector3<f32>
{
    na::Vector3::new(x,y,z)
}

#[allow(unused_imports)]
#[allow(unused_attributes)]
#[cfg(test)]
mod macro_test
{
    extern crate nalgebra as na;
    
    use scad_element::*;
    use scad_object::*;
    use scad_element::ScadElement::*;
    use scad_element::CircleType::*;

    #[macro_use]
    use scad_macros::*;

    #[test]
    fn vec3_test()
    {
        assert_eq!(vec3(0.0, 1.0, 2.0), na::Vector3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn scad_macro_test()
    {
        assert_eq!(scad!(Cube(vec3(1.0,3.0,4.0))).get_code(), "cube([1,3,4]);");

        assert_eq!(scad!(Cube(vec3(1.0,3.0,4.0)); scad!(Cylinder(5.0, Radius(3.0)))).get_code(), "cube([1,3,4])\n{\n\tcylinder(h=5,r=3);\n}");
        
    }

    #[test]
    fn recursive_macro_test()
    {
        let test_element = scad!(Cube(vec3(5.0, 3.0, 3.0)));

        assert_eq!(scad!(
            Translate(vec3(1.0, 3.0, 4.0))
            {
                (Cylinder(1.0, Diameter(3.0)))
                {Cylinder(1.0, Diameter(3.0))}
                [test_element]
            }).get_code(), "translate([1,3,4])\n{\n\tcylinder(h=1,d=3);\n\tcylinder(h=1,d=3);\n\tcube([5,3,3]);\n}");

        let test_element = scad!(Cube(vec3(5.0, 3.0, 3.0)));
        assert_eq!(scad!(
            Translate(vec3(1.0, 3.0, 4.0))
            {
                (Cylinder(2.0, Diameter(3.0)))
                [test_element]
                (Cylinder(1.0, Diameter(3.0)))
            }).get_code(), "translate([1,3,4])\n{\n\tcylinder(h=2,d=3);\n\tcube([5,3,3]);\n\tcylinder(h=1,d=3);\n}");

        //Assuming this generates the correct code, just making sure it compiles
        scad!(
        Translate(vec3(1.0, 3.0, 4.0))
        {
            (Cylinder(2.0, Diameter(3.0)))
            [scad!(Translate(vec3(1.0, 1.0, 1.0))
            {
                (Cube(vec3(1.0, 2.0, 2.0)))
            })]
        });

        scad!(Translate(vec3(1.0, 3.0, 4.0))
        {
            scad!(Translate(vec3(2.0, 1.0, 3.0))
            {
                (Cube(vec3(1.0, 1.0, 1.0)))
            })
            (Union)
        });
    }

    #[test]
    fn many_children_test()
    {
        assert_eq!(scad!(Translate(vec3(0.0,0.0,0.0));{
                scad!(Cube(vec3(1.0,1.0,1.0))),
                scad!(Cube(vec3(1.0,1.0,1.0)))
            }).get_code(), "translate([0,0,0])\n{\n\tcube([1,1,1]);\n\tcube([1,1,1]);\n}"
        );
        //Test trailing edge ,
        assert_eq!(scad!(Translate(vec3(0.0,0.0,0.0));{
                scad!(Cube(vec3(1.0,1.0,1.0))),
                scad!(Cube(vec3(1.0,1.0,1.0))),
            }).get_code(), "translate([0,0,0])\n{\n\tcube([1,1,1]);\n\tcube([1,1,1]);\n}"
        );
    }
}
