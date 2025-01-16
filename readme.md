## Plans:
- Read in the tcx file into memory
  It's an XML file in the end
- Once we have the file into memory we will have a path/profile in GPS coordinates
- Need to maybe convert that into some other value? need to scale it because GPS
are in polar coordinates (or something similar) but I'm going to want to
have it scaled to some cartesian coords scaled to the size of the model in X/Y/Z

- Once converted, we will want to construct the facets
Maybe I should consider making a manim animation of this process?
or use the JS visualization library

### XML Parsing:
the xml-rs library uses a stream reader
Use a nested set of `XmlEvent::StartElement`,`XmlEvent::EndElement` and `XmlEvent::Characters`
Need to figure out the correct way in rust to do this, could easily look
like a nightmare


## STLs
Contain a list of triangles, points should be specified in counter clockwise order,
and a normal for that facet.

will need to create a function to calculate a normal vector for 3 points in 3d space
  (dot product I think of the two vectors formed from the 3 points)
  (making sure to respect right hand rule)

- take the point and point + 1
- Calculate the unit vector of those points
- Triangle 1:
  -point -desired_path_width / 2.0,point1.y, should be orthogonal to unit vector calculated
  -point +desired_path_width / 2.0,point1.y
  -point +desired_path_width
- 

## User Input
The user inputs:
  1. The Input file we want to parse
  2. The output file we want to spit out (optional?)
  3. the desired total dimensions of the stl to scale to: length,width,heigh
  4. The desired width of the path in the STL
