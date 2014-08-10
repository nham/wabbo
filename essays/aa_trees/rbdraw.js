/*
     every node in a tree except the root has a parent.
      - a node's parent is called its 1-parent
      - a node's grandparent is called its 2-parent
      - in general, the parent of a node's n-parent is its (n+1)-parent

      - nodes who share the same 1-parent are called 1-siblings
      - nodes that are not 1-siblings that share the same 2-parent are called 
        2-siblings
      - in general, nodes that are not 1-, 2-, ..., (n-1)-siblings that share the  
        same n-parent are called n-siblings

     this function gives the spacing between two adjacent leaves that are 
     n-siblings, for any n. what are adjacent leaves? if we number the nodes in 
     a binary tree like so:

          1
       2     3
      4 5   6 7

     (and so on for larger complete binary trees), then leaves are adjacent if
     they are successive pairs of numbers. so 

      - 4 & 5 and 6 & 7 are 1-siblings
      - 5 & 6 are 2-siblings
*/
/*
    If we are given 

    - (x, y), the coordinates of the center of the root node
    - v, the vertical distance between levels in the binary tree
    - f: uint -> f64, where f(n) is the distance between adjacent leaves that
      are n-siblings
    - r, the radius of each node
    - d, the number of levels in the binary tree

    then we would like to determine the coordinates for the center of each node
    in the tree.

    we use the standard svg coordinate system (arrows show increasing x and y):

      x
    ------->
    |
    |y
    |
    v

    the strategy is the calculate all leaf positions first, then parent x-coords
    are the averages of x-coords for nodes that have already been calculated
*/
var get_node_coords = function(x, y, d, r, f, v) {
    var left_subtree_width = 0;
    for (var i = 1; i < d-1; i++) {
        left_subtree_width = 2 * left_subtree_width + f(i) + 2*r;
    }

    // level 1 is root, level 2 is root's children, etc.
    // using the numbering scheme of root = 1, and for each level starting at very
    // left and working way to right:
    //          1
    //       2     3
    //      4 5   6 7
    //      ...
    // then level k starts at index 2^{k-1} and contains 2^{k-1} nodes
    var get_level_start = function(k) {
        return Math.pow(2, k-1);
    };

    var get_level_width = function(k) {
        return get_level_start(k); //coincidentally, these are the same!
    }

    //list of coordinates for the center of each node. what we're trying to calculate
    var poses = [];
    for(var i = 0; i < Math.pow(2, d); i++) {
        poses.push(null);
    }
    // we dont use poses[0]
    poses[1] = {x: x, y: y};


    // for each node index k, returns the n such that (k-1) and k are n-siblings
    // only designed to be used for leaf positions
    var sibling_num = function(k) {
        if(k % 2 == 1) {
            return 1;
        } else {
            var a = Math.pow(2, d - 1);
            var b = d;

            while(true) { // this will at least return when we get down to x == 2
                if ( k % a == 0 ) {
                    return b;
                } else {
                    a = a/2;
                    b -= 1;
                }
            }
        }
    };

    // calculate leaf positions
    var bottom_y = y + v*(d-1);
    var i = get_level_start(d);
    poses[i] = {x: x - left_subtree_width - (f(d - 1)/2 + r), 
                y: bottom_y};

    for (var j = 1; j < get_level_width(d); j++) {
        poses[i+1] = {x: poses[i].x + f(sibling_num(i+1)) + 2*r,
                      y: bottom_y};
        i += 1;
    }

    // now the rest of the levels, working our way up
    for (var j = d - 1; j > 1; j--) {
        var this_y = y  + v*(j-1);
        var start = get_level_start(j);
        for(var k = start; k < 2*start; k++) {
            poses[k] = {x: (poses[2*k].x + poses[2*k + 1].x)/2,
                        y: this_y};
        }
    }

    return poses;
};


var draw_rbtree = function(s, x, y, d, r, f, v, nodes) {
    var poses = get_node_coords(x, y, d, r, f, v);

    var try_connect_nodes = function(i, j) {
        if (nodes[i] !== null && nodes[j] !== null) {
            var ipos = poses[i];
            var jpos = poses[j];
            var p = s.line(ipos.x, ipos.y, jpos.x, jpos.y);
            p.attr({
                fill: "#000"
               ,stroke: "#000"
               ,strokeWidth: 2
            });
        }
    };

    for(var i = 1; i < poses.length - (poses.length + 1)/2; i++) {
        try_connect_nodes(i, 2*i);
        try_connect_nodes(i, 2*i + 1);
    }

    for(var i = 1; i < poses.length; i++) {
        if ( nodes[i] !== null ) {
            var node_color;
            if ( nodes[i].color === "r" ) {
                node_color = "#d93232";
            } else {
                node_color = "#444444";
            }

            var bigCircle = s.circle(poses[i].x, poses[i].y, r);
            bigCircle.attr({
                fill: node_color
               ,stroke: node_color
               ,strokeWidth: 3
            });

            var n = nodes[i].text.length;
            var t = s.text(poses[i].x - 5*n, poses[i].y + 5, nodes[i].text);
            t.attr({
                fill: "#fff"
               ,stroke: "#fff"
               ,"font-family": "monospace"
               ,"font-size": "16px"
               ,"font-weight": "normal"
            });

        }
    }
};


